mod init;

extern crate alloc;
use alloc::{boxed::Box, string::ToString};
use alloc::sync::Arc;

use basic::bus::mmio::VirtioMmioDeviceType;
use corelib::AlienResult;
use domain_helper::alloc_domain_id;
use interface::*;
use log::warn;
use shared_heap::DVec;

use crate::{
    create_domain,
    domain::init::init_domains,
    domain_helper,
    domain_helper::{DOMAIN_DATA_ALLOCATOR, SHARED_HEAP_ALLOCATOR},
    domain_loader::creator::*,
    domain_proxy::*,
    mmio_bus, pci_bus, platform_bus, register_domain,
};

/// set the kernel to the specific domain
fn init_kernel_domain() {
    shared_heap::init(SHARED_HEAP_ALLOCATOR, alloc_domain_id());
    storage::init_data_allocator(DOMAIN_DATA_ALLOCATOR);
}

#[cfg(target_arch = "riscv64")]
fn init_device() -> AlienResult<Arc<dyn PLICDomain>> {
    let platform_bus = platform_bus!();
    let mut has_gpu_alias = false;
    let mut input_alias_count = 0usize;

    let plic_device = platform_bus
        .common_devices()
        .iter()
        .find(|device| device.name() == "plic")
        .expect("plic device not found");

    let (plic, domain_file_info) =
        create_domain!(PLICDomainProxy, DomainTypeRaw::PLICDomain, "plic")?;
    let plic_address = plic_device.address_range();
    let plic_info = PlicInfo {
        device_info: plic_address.start.as_usize()..plic_address.end.as_usize(),
        #[cfg(all(target_arch = "riscv64", plat_qemu_riscv))]
        ty: PlicType::Qemu,
        #[cfg(all(target_arch = "riscv64", plat_vf2))]
        ty: PlicType::SiFive,
    };
    plic.init_by_box(Box::new(plic_info))?;
    register_domain!(
        "plic",
        domain_file_info,
        DomainType::PLICDomain(plic.clone()),
        true
    );

    let mut nic_irq = 0;

    for device in platform_bus.common_devices().iter() {
        let address = device.address().as_usize();
        let size = device.io_region().size();
        let irq = device.irq();
        match device.name() {
            "rtc" => {
                if let Some(compatible) = device.compatible() {
                    if compatible != "google,goldfish-rtc" {
                        println_color!(31, "unknown rtc device: {}", compatible);
                        continue;
                    }
                }
                let (rtc, domain_file_info) =
                    create_domain!(RtcDomainProxy, DomainTypeRaw::RtcDomain, "goldfish")?;
                rtc.init_by_box(Box::new(address..address + size))?;
                register_domain!("rtc", domain_file_info, DomainType::RtcDomain(rtc), true);
                plic.register_irq(irq.unwrap() as _, &DVec::from_slice("rtc".as_bytes()))?;
            }
            "uart" => {
                let compatible = device
                    .compatible()
                    .expect("uart device must have compatible property");
                let (uart, domain_file_info) = match compatible {
                    "ns16550a" => {
                        create_domain!(UartDomainProxy, DomainTypeRaw::UartDomain, "uart16550")?
                    }
                    "snps,dw-apb-uart" => {
                        create_domain!(UartDomainProxy, DomainTypeRaw::UartDomain, "uart8250")?
                    }
                    _ => panic!("unknown uart device: {}", compatible),
                };

                uart.init_by_box(Box::new(address..address + size))?;
                register_domain!("uart", domain_file_info, DomainType::UartDomain(uart), true);

                let (buf_uart, domain_file_info) =
                    create_domain!(BufUartDomainProxy, DomainTypeRaw::BufUartDomain, "buf_uart")?;
                buf_uart.init_by_box(Box::new("uart".to_string()))?;
                register_domain!(
                    "buf_uart",
                    domain_file_info,
                    DomainType::BufUartDomain(buf_uart),
                    true
                );

                plic.register_irq(irq.unwrap() as _, &DVec::from_slice("buf_uart".as_bytes()))?;
            }
            "sdcard" => {
                let (sdcard, domain_file_info) =
                    create_domain!(BlkDomainProxy, DomainTypeRaw::BlkDeviceDomain, "vf2_sd")?;
                sdcard.init_by_box(Box::new(VirtioInitInfo::mmio(
                    address..address + size,
                    irq,
                )))?;
                register_domain!(
                    "block",
                    domain_file_info,
                    DomainType::BlkDeviceDomain(sdcard),
                    false
                );
            }
            _ => {
                warn!("unknown device: {}", device.name());
            }
        }
    }

    for device in mmio_bus!().lock().common_devices().iter() {
        let address = device.address().as_usize();
        let size = device.io_region().size();
        match device.device_type() {
            VirtioMmioDeviceType::Network => {
                let (net_driver, domain_file_info) = create_domain!(
                    NetDeviceDomainProxy,
                    DomainTypeRaw::NetDeviceDomain,
                    "virtio_net"
                )?;
                net_driver.init_by_box(Box::new(VirtioInitInfo::mmio(
                    address..address + size,
                    device.irq(),
                )))?;
                let net_domain = DomainType::NetDeviceDomain(net_driver.clone());
                register_domain!(
                    "virtio_net",
                    domain_file_info.clone(),
                    net_domain.clone(),
                    false
                );
                register_domain!(
                    "nic",
                    domain_file_info,
                    net_domain,
                    false
                );
            }
            VirtioMmioDeviceType::Block => {
                let (blk_driver, domain_file_info) = create_domain!(
                    BlkDomainProxy,
                    DomainTypeRaw::BlkDeviceDomain,
                    "virtio_blk"
                )?;
                blk_driver.init_by_box(Box::new(VirtioInitInfo::mmio(
                    address..address + size,
                    device.irq(),
                )))?;
                let blk_domain = DomainType::BlkDeviceDomain(blk_driver.clone());
                register_domain!(
                    "virtio_block",
                    domain_file_info.clone(),
                    blk_domain.clone(),
                    false
                );
                register_domain!(
                    "block",
                    domain_file_info,
                    blk_domain,
                    false
                );
            }
            VirtioMmioDeviceType::Input => {
                let (input_driver, domain_file_info) = create_domain!(
                    InputDomainProxy,
                    DomainTypeRaw::InputDomain,
                    "virtio_input"
                )?;
                input_driver.init_by_box(Box::new(VirtioInitInfo::mmio(
                    address..address + size,
                    device.irq(),
                )))?;
                let input_name = register_domain!(
                    "virtio_input",
                    domain_file_info,
                    DomainType::InputDomain(input_driver),
                    false
                );
                let (buf_input, domain_file_info) = create_domain!(
                    BufInputDomainProxy,
                    DomainTypeRaw::BufInputDomain,
                    "buf_input"
                )?;
                assert!(input_name.starts_with("virtio_input-"));
                buf_input.init_by_box(Box::new(input_name))?;
                let buf_input_domain = DomainType::BufInputDomain(buf_input.clone());
                let buf_input_name = register_domain!(
                    "buf_input",
                    domain_file_info.clone(),
                    buf_input_domain.clone(),
                    false
                );
                assert!(buf_input_name.starts_with("buf_input-"));
                if input_alias_count == 0 {
                    register_domain!(
                        "keyboard",
                        domain_file_info.clone(),
                        buf_input_domain.clone(),
                        true
                    );
                } else if input_alias_count == 1 {
                    register_domain!("mouse", domain_file_info, buf_input_domain, true);
                }
                input_alias_count += 1;
            }
            VirtioMmioDeviceType::GPU => {
                let (gpu_driver, domain_file_info) =
                    create_domain!(GpuDomainProxy, DomainTypeRaw::GpuDomain, "virtio_gpu")?;
                gpu_driver.init_by_box(Box::new(VirtioInitInfo::mmio(
                    address..address + size,
                    device.irq(),
                )))?;
                let gpu_domain = DomainType::GpuDomain(gpu_driver.clone());
                register_domain!(
                    "virtio_gpu",
                    domain_file_info.clone(),
                    gpu_domain.clone(),
                    false
                );
                if !has_gpu_alias {
                    register_domain!("gpu", domain_file_info, gpu_domain, true);
                    has_gpu_alias = true;
                }
            }
            _ => {
                warn!("unknown device: {:?}", device.device_type());
            }
        }
    }

    {
        let (net_stack, domain_file_info) =
            create_domain!(NetDomainProxy, DomainTypeRaw::NetDomain, "net_stack")?;
        net_stack.init_by_box(Box::new("nic-1".to_string()))?;
        register_domain!(
            "net_stack",
            domain_file_info,
            DomainType::NetDomain(net_stack),
            true
        );
    }

    let (shadow_blk, domain_file_info) = create_domain!(
        ShadowBlockDomainProxy,
        DomainTypeRaw::ShadowBlockDomain,
        "shadow_blk"
    )?;
    shadow_blk.init_by_box(Box::new("block-1".to_string()))?;
    register_domain!(
        "shadow_blk",
        domain_file_info,
        DomainType::ShadowBlockDomain(shadow_blk),
        false
    );
    let (cache_blk, domain_file_info) = create_domain!(
        CacheBlkDomainProxy,
        DomainTypeRaw::CacheBlkDeviceDomain,
        "cache_blk"
    )?;
    cache_blk.init_by_box(Box::new("shadow_blk-1".to_string()))?;
    register_domain!(
        "cache_blk",
        domain_file_info,
        DomainType::CacheBlkDeviceDomain(cache_blk),
        false
    );

    let (null_device, domain_file_info) =
        create_domain!(EmptyDeviceDomainProxy, DomainTypeRaw::EmptyDeviceDomain, "null")?;
    null_device.init_by_box(Box::new(()))?;
    register_domain!(
        "null",
        domain_file_info,
        DomainType::EmptyDeviceDomain(null_device),
        true
    );
    let (random_device, domain_file_info) =
        create_domain!(EmptyDeviceDomainProxy, DomainTypeRaw::EmptyDeviceDomain, "random")?;
    random_device.init_by_box(Box::new(()))?;
    register_domain!(
        "random",
        domain_file_info,
        DomainType::EmptyDeviceDomain(random_device),
        true
    );

    Ok(plic)
}

#[cfg(target_arch = "x86_64")]
fn init_device() -> AlienResult<Arc<dyn APICDomain>> {
    let platform_bus = platform_bus!();
    let mut has_nic = false;
    let mut has_block = false;
    let mut has_gpu_alias = false;
    let mut input_alias_count = 0usize;

    let (apic, domain_file_info) = create_domain!(
        APICDomainProxy,
        DomainTypeRaw::APICDomain,
        "apic"
    )?;
    apic.init_by_box(Box::new(()))?;
    register_domain!(
        "apic",
        domain_file_info,
        DomainType::APICDomain(apic.clone()),
        true
    );

    for device in platform_bus.common_devices().iter() {
        let address = device.address().as_usize();
        let size = device.io_region().size();
        let irq = device.irq();

        match device.name() {
            "local_apic" => {
                let (local_apic, domain_file_info) = create_domain!(
                    EmptyDeviceDomainProxy,
                    DomainTypeRaw::EmptyDeviceDomain,
                    "local_apic"
                )?;
                local_apic.init_by_box(Box::new(address..address + size))?;
                register_domain!(
                    "local_apic",
                    domain_file_info,
                    DomainType::EmptyDeviceDomain(local_apic),
                    true
                );
            }
            "io_apic" => {
                let (io_apic, domain_file_info) = create_domain!(
                    EmptyDeviceDomainProxy,
                    DomainTypeRaw::EmptyDeviceDomain,
                    "io_apic"
                )?;
                io_apic.init_by_box(Box::new(address..address + size))?;
                register_domain!(
                    "io_apic",
                    domain_file_info,
                    DomainType::EmptyDeviceDomain(io_apic),
                    true
                );
            }
            "uart" => {
                let compatible = device
                    .compatible()
                    .expect("uart device must have compatible property");
                let (uart, domain_file_info) = match compatible {
                    "ns16550a" => {
                        create_domain!(UartDomainProxy, DomainTypeRaw::UartDomain, "uart16550")?
                    }
                    "snps,dw-apb-uart" => {
                        create_domain!(UartDomainProxy, DomainTypeRaw::UartDomain, "uart8250")?
                    }
                    _ => panic!("unknown uart device: {}", compatible),
                };

                uart.init_by_box(Box::new(address..address + size))?;
                register_domain!("uart", domain_file_info, DomainType::UartDomain(uart), true);

                let (buf_uart, domain_file_info) =
                    create_domain!(BufUartDomainProxy, DomainTypeRaw::BufUartDomain, "buf_uart")?;
                buf_uart.init_by_box(Box::new("uart".to_string()))?;
                let buf_uart_name = register_domain!(
                    "buf_uart",
                    domain_file_info,
                    DomainType::BufUartDomain(buf_uart),
                    true
                );

                if let Some(irq) = irq {
                    let vector = 32u8.saturating_add(irq as u8);
                    platform::apic::configure_irq(irq as u8, vector, 0);
                    platform::apic::set_irq_enable(irq as usize, true);
                    apic.register_irq(irq as _, &DVec::from_slice(buf_uart_name.as_bytes()))?;
                }
            }
            "rtc" => {
                // 当前 x86_64 RTC 仅完成总线枚举，域驱动后续再接入。
                warn!("rtc device detected, rtc domain is not wired yet");
            }
            "ramdisk" => {
                let (ramdisk, domain_file_info) =
                    create_domain!(BlkDomainProxy, DomainTypeRaw::BlkDeviceDomain, "mem_block")?;
                ramdisk.init_by_box(Box::new(VirtioInitInfo::mmio(
                    address..address + size,
                    irq,
                )))?;
                #[cfg(not(feature = "bench"))]
                register_domain!(
                    "block",
                    domain_file_info,
                    DomainType::BlkDeviceDomain(ramdisk),
                    false
                );
                has_block = true;
                #[cfg(feature = "bench")]
                register_domain!(
                    "bench_block",
                    domain_file_info,
                    DomainType::BlkDeviceDomain(ramdisk),
                    true
                );
            }
            "pci_ecam" => {
                // x86_64 的 virtio PCI 统一在平台设备遍历后处理，这里保留分支用于兼容旧日志路径。
            }
            _ => {
                warn!("unknown device: {}", device.name());
            }
        }
    }

    #[cfg(target_arch = "x86_64")]
    {
        // x86_64 不依赖 platform_bus 是否暴露 pci_ecam，统一按 PCI endpoint 绑定 virtio 域。
        let (blk_ep, net_ep, input_eps, gpu_ep) = {
            let bus = pci_bus!().lock();
            let blk_ep = bus
                .endpoint_devices()
                .iter()
                .find(|ep| ep.virtio_kind() == Some("virtio-blk"))
                .copied();
            let net_ep = bus
                .endpoint_devices()
                .iter()
                .find(|ep| ep.virtio_kind() == Some("virtio-net"))
                .copied();
            let input_eps = bus
                .endpoint_devices()
                .iter()
                .filter(|ep| ep.virtio_kind() == Some("virtio-input"))
                .copied()
                .collect::<alloc::vec::Vec<_>>();
            let gpu_ep = bus
                .endpoint_devices()
                .iter()
                .find(|ep| ep.virtio_kind() == Some("virtio-gpu"))
                .copied();
            (blk_ep, net_ep, input_eps, gpu_ep)
        };

        if let Some(ep) = blk_ep {
            let bdf = ep.address();
            let legacy_io = ep.legacy_io_range();
            let modern = ep.virtio_modern_info();
            if let Some(modern_info) = modern.as_ref() {
                mem::map_device_phys_range(modern_info.common.clone());
                mem::map_device_phys_range(modern_info.notify.clone());
                mem::map_device_phys_range(modern_info.isr.clone());
                mem::map_device_phys_range(modern_info.device.clone());
            }
            if legacy_io.is_some() || modern.is_some() {
                let init_info = VirtioInitInfo::pci(
                    bdf.segment(),
                    bdf.bus(),
                    bdf.device(),
                    bdf.function(),
                    None,
                    legacy_io,
                )
                .with_modern_pci(
                    modern.as_ref().map(|x| x.common.clone()),
                    modern.as_ref().map(|x| x.notify.clone()),
                    modern.as_ref().map(|x| x.notify_off_multiplier),
                    modern.as_ref().map(|x| x.isr.clone()),
                    modern.as_ref().map(|x| x.device.clone()),
                );
                let (blk_driver, domain_file_info) = create_domain!(
                    BlkDomainProxy,
                    DomainTypeRaw::BlkDeviceDomain,
                    "virtio_blk"
                )?;
                blk_driver.init_by_box(Box::new(init_info))?;
                println!(
                    "virtio-pci block @ {:04x}:{:02x}:{:02x}.{}",
                    bdf.segment(),
                    bdf.bus(),
                    bdf.device(),
                    bdf.function()
                );
                let blk_domain = DomainType::BlkDeviceDomain(blk_driver.clone());
                register_domain!(
                    "virtio_block",
                    domain_file_info.clone(),
                    blk_domain.clone(),
                    false
                );
                register_domain!(
                    "block",
                    domain_file_info,
                    blk_domain,
                    false
                );
                has_block = true;
            } else {
                warn!(
                    "virtio-blk @ {:04x}:{:02x}:{:02x}.{} has no usable transport (legacy/modern), skip",
                    bdf.segment(),
                    bdf.bus(),
                    bdf.device(),
                    bdf.function()
                );
            }
        }

        if let Some(ep) = net_ep {
            let bdf = ep.address();
            let legacy_io = ep.legacy_io_range();
            let modern = ep.virtio_modern_info();
            if let Some(modern_info) = modern.as_ref() {
                mem::map_device_phys_range(modern_info.common.clone());
                mem::map_device_phys_range(modern_info.notify.clone());
                mem::map_device_phys_range(modern_info.isr.clone());
                mem::map_device_phys_range(modern_info.device.clone());
            }
            if legacy_io.is_some() || modern.is_some() {
                let init_info = VirtioInitInfo::pci(
                    bdf.segment(),
                    bdf.bus(),
                    bdf.device(),
                    bdf.function(),
                    None,
                    legacy_io,
                )
                .with_modern_pci(
                    modern.as_ref().map(|x| x.common.clone()),
                    modern.as_ref().map(|x| x.notify.clone()),
                    modern.as_ref().map(|x| x.notify_off_multiplier),
                    modern.as_ref().map(|x| x.isr.clone()),
                    modern.as_ref().map(|x| x.device.clone()),
                );
                let (net_driver, domain_file_info) = create_domain!(
                    NetDeviceDomainProxy,
                    DomainTypeRaw::NetDeviceDomain,
                    "virtio_net"
                )?;
                net_driver.init_by_box(Box::new(init_info))?;
                let net_domain = DomainType::NetDeviceDomain(net_driver.clone());
                register_domain!(
                    "virtio_net",
                    domain_file_info.clone(),
                    net_domain.clone(),
                    false
                );
                register_domain!(
                    "nic",
                    domain_file_info,
                    net_domain,
                    false
                );
                has_nic = true;
            } else {
                warn!(
                    "virtio-net @ {:04x}:{:02x}:{:02x}.{} has no usable transport (legacy/modern), skip",
                    bdf.segment(),
                    bdf.bus(),
                    bdf.device(),
                    bdf.function()
                );
            }
        }

        for ep in input_eps {
            let bdf = ep.address();
            let legacy_io = ep.legacy_io_range();
            let modern = ep.virtio_modern_info();
            if let Some(modern_info) = modern.as_ref() {
                mem::map_device_phys_range(modern_info.common.clone());
                mem::map_device_phys_range(modern_info.notify.clone());
                mem::map_device_phys_range(modern_info.isr.clone());
                mem::map_device_phys_range(modern_info.device.clone());
            }
            if legacy_io.is_some() || modern.is_some() {
                let init_info = VirtioInitInfo::pci(
                    bdf.segment(),
                    bdf.bus(),
                    bdf.device(),
                    bdf.function(),
                    None,
                    legacy_io,
                )
                .with_modern_pci(
                    modern.as_ref().map(|x| x.common.clone()),
                    modern.as_ref().map(|x| x.notify.clone()),
                    modern.as_ref().map(|x| x.notify_off_multiplier),
                    modern.as_ref().map(|x| x.isr.clone()),
                    modern.as_ref().map(|x| x.device.clone()),
                );
                let (input_driver, domain_file_info) = create_domain!(
                    InputDomainProxy,
                    DomainTypeRaw::InputDomain,
                    "virtio_input"
                )?;
                input_driver.init_by_box(Box::new(init_info))?;
                let input_name = register_domain!(
                    "virtio_input",
                    domain_file_info,
                    DomainType::InputDomain(input_driver),
                    false
                );
                let (buf_input, domain_file_info) = create_domain!(
                    BufInputDomainProxy,
                    DomainTypeRaw::BufInputDomain,
                    "buf_input"
                )?;
                buf_input.init_by_box(Box::new(input_name))?;
                let buf_input_domain = DomainType::BufInputDomain(buf_input.clone());
                register_domain!(
                    "buf_input",
                    domain_file_info.clone(),
                    buf_input_domain.clone(),
                    false
                );
                if input_alias_count == 0 {
                    register_domain!(
                        "keyboard",
                        domain_file_info.clone(),
                        buf_input_domain.clone(),
                        true
                    );
                } else if input_alias_count == 1 {
                    register_domain!("mouse", domain_file_info, buf_input_domain, true);
                }
                input_alias_count += 1;
            } else {
                warn!(
                    "virtio-input @ {:04x}:{:02x}:{:02x}.{} has no usable transport (legacy/modern), skip",
                    bdf.segment(),
                    bdf.bus(),
                    bdf.device(),
                    bdf.function()
                );
            }
        }

        if let Some(ep) = gpu_ep {
            let bdf = ep.address();
            let legacy_io = ep.legacy_io_range();
            let modern = ep.virtio_modern_info();
            if let Some(modern_info) = modern.as_ref() {
                mem::map_device_phys_range(modern_info.common.clone());
                mem::map_device_phys_range(modern_info.notify.clone());
                mem::map_device_phys_range(modern_info.isr.clone());
                mem::map_device_phys_range(modern_info.device.clone());
            }
            if legacy_io.is_some() || modern.is_some() {
                let init_info = VirtioInitInfo::pci(
                    bdf.segment(),
                    bdf.bus(),
                    bdf.device(),
                    bdf.function(),
                    None,
                    legacy_io,
                )
                .with_modern_pci(
                    modern.as_ref().map(|x| x.common.clone()),
                    modern.as_ref().map(|x| x.notify.clone()),
                    modern.as_ref().map(|x| x.notify_off_multiplier),
                    modern.as_ref().map(|x| x.isr.clone()),
                    modern.as_ref().map(|x| x.device.clone()),
                );
                let (gpu_driver, domain_file_info) =
                    create_domain!(GpuDomainProxy, DomainTypeRaw::GpuDomain, "virtio_gpu")?;
                gpu_driver.init_by_box(Box::new(init_info))?;
                let gpu_domain = DomainType::GpuDomain(gpu_driver.clone());
                register_domain!(
                    "virtio_gpu",
                    domain_file_info.clone(),
                    gpu_domain.clone(),
                    false
                );
                if !has_gpu_alias {
                    register_domain!("gpu", domain_file_info, gpu_domain, true);
                    has_gpu_alias = true;
                }
            } else {
                warn!(
                    "virtio-gpu @ {:04x}:{:02x}:{:02x}.{} has no usable transport (legacy/modern), skip",
                    bdf.segment(),
                    bdf.bus(),
                    bdf.device(),
                    bdf.function()
                );
            }
        }
    }

    for device in mmio_bus!().lock().common_devices().iter() {
        let address = device.address().as_usize();
        let size = device.io_region().size();
        match device.device_type() {
            VirtioMmioDeviceType::Network => {
                let (net_driver, domain_file_info) = create_domain!(
                    NetDeviceDomainProxy,
                    DomainTypeRaw::NetDeviceDomain,
                    "virtio_net"
                )?;
                net_driver.init_by_box(Box::new(VirtioInitInfo::mmio(
                    address..address + size,
                    device.irq(),
                )))?;
                let net_domain = DomainType::NetDeviceDomain(net_driver.clone());
                register_domain!(
                    "virtio_net",
                    domain_file_info.clone(),
                    net_domain.clone(),
                    false
                );
                register_domain!(
                    "nic",
                    domain_file_info,
                    net_domain,
                    false
                );
                has_nic = true;
            }
            VirtioMmioDeviceType::Block => {
                let (blk_driver, domain_file_info) = create_domain!(
                    BlkDomainProxy,
                    DomainTypeRaw::BlkDeviceDomain,
                    "virtio_blk"
                )?;
                blk_driver.init_by_box(Box::new(VirtioInitInfo::mmio(
                    address..address + size,
                    device.irq(),
                )))?;
                let blk_domain = DomainType::BlkDeviceDomain(blk_driver.clone());
                register_domain!(
                    "virtio_block",
                    domain_file_info.clone(),
                    blk_domain.clone(),
                    false
                );
                register_domain!(
                    "block",
                    domain_file_info,
                    blk_domain,
                    false
                );
                has_block = true;
            }
            VirtioMmioDeviceType::Input => {
                let (input_driver, domain_file_info) = create_domain!(
                    InputDomainProxy,
                    DomainTypeRaw::InputDomain,
                    "virtio_input"
                )?;
                input_driver.init_by_box(Box::new(VirtioInitInfo::mmio(
                    address..address + size,
                    device.irq(),
                )))?;
                let input_name = register_domain!(
                    "virtio_input",
                    domain_file_info,
                    DomainType::InputDomain(input_driver),
                    false
                );
                let (buf_input, domain_file_info) = create_domain!(
                    BufInputDomainProxy,
                    DomainTypeRaw::BufInputDomain,
                    "buf_input"
                )?;
                assert!(input_name.starts_with("virtio_input-"));
                buf_input.init_by_box(Box::new(input_name))?;
                let buf_input_domain = DomainType::BufInputDomain(buf_input.clone());
                let buf_input_name = register_domain!(
                    "buf_input",
                    domain_file_info.clone(),
                    buf_input_domain.clone(),
                    false
                );
                assert!(buf_input_name.starts_with("buf_input-"));
                if input_alias_count == 0 {
                    register_domain!(
                        "keyboard",
                        domain_file_info.clone(),
                        buf_input_domain.clone(),
                        true
                    );
                } else if input_alias_count == 1 {
                    register_domain!("mouse", domain_file_info, buf_input_domain, true);
                }
                input_alias_count += 1;
            }
            VirtioMmioDeviceType::GPU => {
                let (gpu_driver, domain_file_info) =
                    create_domain!(GpuDomainProxy, DomainTypeRaw::GpuDomain, "virtio_gpu")?;
                gpu_driver.init_by_box(Box::new(VirtioInitInfo::mmio(
                    address..address + size,
                    device.irq(),
                )))?;
                let gpu_domain = DomainType::GpuDomain(gpu_driver.clone());
                register_domain!(
                    "virtio_gpu",
                    domain_file_info.clone(),
                    gpu_domain.clone(),
                    false
                );
                if !has_gpu_alias {
                    register_domain!("gpu", domain_file_info, gpu_domain, true);
                    has_gpu_alias = true;
                }
            }
            _ => {
                warn!("unknown device: {:?}", device.device_type());
            }
        }
    }

    if has_nic {
        let (net_stack, domain_file_info) =
            create_domain!(NetDomainProxy, DomainTypeRaw::NetDomain, "net_stack")?;
        net_stack.init_by_box(Box::new("nic-1".to_string()))?;
        register_domain!(
            "net_stack",
            domain_file_info,
            DomainType::NetDomain(net_stack),
            true
        );
    } else {
        // 最小启动链路下可能没有网卡，跳过 net_stack 以继续引导。
        warn!("nic device not found, skip net_stack init");
    }

    if has_block {
        let (shadow_blk, domain_file_info) = create_domain!(
            ShadowBlockDomainProxy,
            DomainTypeRaw::ShadowBlockDomain,
            "shadow_blk"
        )?;
        shadow_blk.init_by_box(Box::new("block-1".to_string()))?;
        register_domain!(
            "shadow_blk",
            domain_file_info,
            DomainType::ShadowBlockDomain(shadow_blk),
            false
        );
        let (cache_blk, domain_file_info) = create_domain!(
            CacheBlkDomainProxy,
            DomainTypeRaw::CacheBlkDeviceDomain,
            "cache_blk"
        )?;
        cache_blk.init_by_box(Box::new("shadow_blk-1".to_string()))?;
        register_domain!(
            "cache_blk",
            domain_file_info,
            DomainType::CacheBlkDeviceDomain(cache_blk),
            false
        );
    } else {
        // 仅 initrd 启动时可能没有块设备，跳过块缓存链路避免启动失败。
        warn!("block device not found, skip shadow_blk/cache_blk init");
    }

    let (null_device, domain_file_info) =
        create_domain!(EmptyDeviceDomainProxy, DomainTypeRaw::EmptyDeviceDomain, "null")?;
    null_device.init_by_box(Box::new(()))?;
    register_domain!(
        "null",
        domain_file_info,
        DomainType::EmptyDeviceDomain(null_device),
        true
    );
    let (random_device, domain_file_info) =
        create_domain!(EmptyDeviceDomainProxy, DomainTypeRaw::EmptyDeviceDomain, "random")?;
    random_device.init_by_box(Box::new(()))?;
    register_domain!(
        "random",
        domain_file_info,
        DomainType::EmptyDeviceDomain(random_device),
        true
    );

    Ok(apic)
}

pub fn load_domains() -> AlienResult<()> {
    init_domains();
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

    // we need to register vfs and task domain before init device, because we need to use vfs and task domain in some
    // device init function
    #[cfg(target_arch = "riscv64")]
    let plic = init_device()?;
    #[cfg(target_arch = "x86_64")]
    let apic = init_device()?;

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
        let data = initrd.as_ref().unwrap();
        platform::println!("load_domains: before vfs init");
        vfs.init_by_box(Box::new(data.as_slice().to_vec()))?;
        platform::println!("load_domains: after vfs init");
        initrd.take(); // release the initrd data
    }

    platform::println!("load_domains: before task init");
    task.init_by_box(Box::new(()))?;
    platform::println!("load_domains: after task init");

    let (syscall, domain_file_info) =
        create_domain!(SysCallDomainProxy, DomainTypeRaw::SysCallDomain, "syscall")?;
    syscall.init_by_box(Box::new(()))?;
    register_domain!(
        "syscall",
        domain_file_info,
        DomainType::SysCallDomain(syscall.clone()),
        true
    );

    platform::println!("Load domains done");

    crate::task::register_task_domain(task);
    crate::trap::register_syscall_domain(syscall);
    #[cfg(target_arch = "riscv64")]
    crate::trap::register_plic_domain(plic);
    #[cfg(target_arch = "x86_64")]
    crate::trap::register_apic_domain(apic);
    platform::println!("Register task domain and syscall domain to trap system");
    Ok(())
}
