mod init;
#[cfg(target_arch = "riscv64")]
mod riscv64;
#[cfg(any(
    all(
        target_arch = "x86_64",
        any(
            feature = "domain_test",
            feature = "domain_syscall_test",
            feature = "domain_task_test",
            feature = "domain_apic_test",
            feature = "domain_uart_test",
            feature = "domain_block_test",
            feature = "domain_net_test",
        )
    ),
    all(target_arch = "riscv64", feature = "domain_net_test"),
))]
mod test;
#[cfg(target_arch = "x86_64")]
mod x86_64;

extern crate alloc;
use alloc::boxed::Box;
use core::ops::Range;

use basic::bus::mmio::VirtioMmioDeviceType;
use corelib::AlienResult;
use domain_helper::alloc_domain_id;
use interface::*;
use log::warn;

#[cfg(target_arch = "riscv64")]
use self::riscv64::init_device;
#[cfg(target_arch = "x86_64")]
use self::x86_64::init_device;
#[cfg(target_arch = "x86_64")]
use self::x86_64::X86ApicDomains;
use crate::domain_proxy::SchedulerDomainProxy;
use crate::{
    bus::DeviceLocator,
    create_domain,
    domain::init::init_domains,
    domain_helper,
    domain_helper::{DOMAIN_DATA_ALLOCATOR, SHARED_HEAP_ALLOCATOR},
    domain_loader::creator::*,
    domain_proxy::*,
    register_domain,
};

/// set the kernel to the specific domain
fn init_kernel_domain() {
    shared_heap::init(SHARED_HEAP_ALLOCATOR, alloc_domain_id());
    storage::init_data_allocator(DOMAIN_DATA_ALLOCATOR);
}

fn require_mmio_range_or_einval(
    arch: &str,
    device_tag: &str,
    locator: &DeviceLocator,
) -> AlienResult<Range<usize>> {
    match locator {
        DeviceLocator::Mmio(range) => Ok(range.start.as_usize()..range.end.as_usize()),
        other => {
            log::error!(
                "[locator][{}][{}] expected MMIO locator, got {:?}, reject with EINVAL",
                arch,
                device_tag,
                other
            );
            Err(crate::error::AlienError::EINVAL)
        }
    }
}

fn try_virtio_mmio_range_or_skip(
    arch: &str,
    device_type: VirtioMmioDeviceType,
    mmio_range: Option<Range<mem::PhysAddr>>,
    locator: &DeviceLocator,
) -> AlienResult<Option<Range<usize>>> {
    if let Some(range) = mmio_range {
        return Ok(Some(range.start.as_usize()..range.end.as_usize()));
    }

    #[cfg(feature = "strict_locator")]
    {
        log::error!(
            "[locator][{}][virtio-mmio:{:?}] expected MMIO locator, got {:?}, strict mode reject with EINVAL",
            arch,
            device_type,
            locator
        );
        return Err(crate::error::AlienError::EINVAL);
    }

    #[cfg(not(feature = "strict_locator"))]
    {
        warn!(
            "[locator][{}][virtio-mmio:{:?}] expected MMIO locator, got {:?}, skip",
            arch, device_type, locator
        );
        Ok(None)
    }
}

pub fn load_domains() -> AlienResult<()> {
    init_domains()?;
    init_kernel_domain();
    domain_helper::init_domain_create(Box::new(DomainCreateImpl));

    let (scheduler, domain_file_info) = create_domain!(
        SchedulerDomainProxy,
        DomainTypeRaw::SchedulerDomain,
        "fifo_scheduler"
    )?;
    scheduler.init_by_box(Box::new(()))?;
    register_domain!(
        "scheduler",
        domain_file_info,
        DomainType::SchedulerDomain(scheduler.clone()),
        true
    );
    crate::task::register_scheduler_domain(scheduler);

    let (logger, domain_file_info) =
        create_domain!(LogDomainProxy, DomainTypeRaw::LogDomain, "logger")?;
    logger.init_by_box(Box::new(()))?;
    register_domain!(
        "logger",
        domain_file_info,
        DomainType::LogDomain(logger),
        true
    );

    let (fatfs, domain_file_info) =
        create_domain!(FsDomainProxy, DomainTypeRaw::FsDomain, "fatfs")?;
    register_domain!(
        "fatfs",
        domain_file_info,
        DomainType::FsDomain(fatfs.clone()),
        false
    );

    let (ramfs, domain_file_info) =
        create_domain!(FsDomainProxy, DomainTypeRaw::FsDomain, "ramfs")?;
    register_domain!(
        "ramfs",
        domain_file_info,
        DomainType::FsDomain(ramfs.clone()),
        false
    );

    let (devfs, domain_file_info) =
        create_domain!(DevFsDomainProxy, DomainTypeRaw::DevFsDomain, "devfs")?;
    register_domain!(
        "devfs",
        domain_file_info,
        DomainType::DevFsDomain(devfs.clone()),
        true
    );

    let (procfs, domain_file_info) =
        create_domain!(FsDomainProxy, DomainTypeRaw::FsDomain, "procfs")?;
    register_domain!(
        "procfs",
        domain_file_info,
        DomainType::FsDomain(procfs.clone()),
        true
    );

    let (sysfs, domain_file_info) =
        create_domain!(FsDomainProxy, DomainTypeRaw::FsDomain, "sysfs")?;
    register_domain!(
        "sysfs",
        domain_file_info,
        DomainType::FsDomain(sysfs.clone()),
        true
    );

    let (pipefs, domain_file_info) =
        create_domain!(FsDomainProxy, DomainTypeRaw::FsDomain, "pipefs")?;
    register_domain!(
        "pipefs",
        domain_file_info,
        DomainType::FsDomain(pipefs.clone()),
        true
    );

    let (domainfs, domain_file_info) =
        create_domain!(FsDomainProxy, DomainTypeRaw::FsDomain, "domainfs")?;
    register_domain!(
        "domainfs",
        domain_file_info,
        DomainType::FsDomain(domainfs.clone()),
        true
    );

    let (vfs, domain_file_info) = create_domain!(VfsDomainProxy, DomainTypeRaw::VfsDomain, "vfs")?;
    register_domain!(
        "vfs",
        domain_file_info,
        DomainType::VfsDomain(vfs.clone()),
        true
    );

    let (task, domain_file_info) =
        create_domain!(TaskDomainProxy, DomainTypeRaw::TaskDomain, "task")?; // ref to scheduler domain
    register_domain!(
        "task",
        domain_file_info,
        DomainType::TaskDomain(task.clone()),
        true
    );

    let (syscall, domain_file_info) =
        create_domain!(SysCallDomainProxy, DomainTypeRaw::SysCallDomain, "syscall")?;
    syscall.init_by_box(Box::new(()))?;
    register_domain!(
        "syscall",
        domain_file_info,
        DomainType::SysCallDomain(syscall.clone()),
        true
    );

    // we need to register vfs and task domain before init device, because we need to use vfs and task domain in some
    // device init function
    #[cfg(target_arch = "x86_64")]
    let X86ApicDomains {
        local_apic,
        io_apic: interrupt_controller,
    } = init_device()?;
    #[cfg(target_arch = "riscv64")]
    let interrupt_controller = init_device()?;

    #[cfg(all(
        target_arch = "x86_64",
        any(
            feature = "domain_test",
            feature = "domain_syscall_test",
            feature = "domain_task_test",
            feature = "domain_apic_test",
            feature = "domain_uart_test",
            feature = "domain_block_test",
            feature = "domain_net_test",
        )
    ))]
    test::run()?;

    // riscv64 默认不跑域测试，仅在显式开启网络回归时触发。
    #[cfg(all(target_arch = "riscv64", feature = "domain_net_test"))]
    test::run()?;

    devfs.init_by_box(Box::new(()))?;
    fatfs.init_by_box(Box::new(()))?;
    ramfs.init_by_box(Box::new(()))?;
    procfs.init_by_box(Box::new(()))?;
    sysfs.init_by_box(Box::new(()))?;
    domainfs.init_by_box(Box::new(()))?;

    // The vfs domain may use the device domain, so we need to init vfs domain after init device domain,
    // also it may use the task domain.
    {
        let mut initrd = mem::INITRD_DATA.lock();
        let data = initrd
            .as_ref()
            .ok_or_else(|| {
                log::error!("load_domains: initrd data missing before vfs init");
                crate::error::AlienError::EINVAL
            })?
            .as_slice()
            .to_vec();
        platform::println!("load_domains: before vfs init");
        vfs.init_by_box(Box::new(data))?;
        platform::println!("load_domains: after vfs init");
        initrd.take(); // release the initrd data
    }

    platform::println!("load_domains: before task init");
    task.init_by_box(Box::new(()))?;
    platform::println!("load_domains: after task init");

    platform::println!("Load domains done");

    crate::task::register_task_domain(task);
    crate::trap::register_syscall_domain(syscall);
    crate::trap::register_interrupt_controller_domain(interrupt_controller);
    #[cfg(target_arch = "x86_64")]
    {
        crate::trap::register_local_apic_domain(local_apic);
    }
    platform::println!("Register task domain and syscall domain to trap system");
    Ok(())
}
