use alloc::{boxed::Box, string::ToString, sync::Arc};

use basic::bus::mmio::VirtioMmioDeviceType;
use corelib::AlienResult;
use interface::*;
use interface::PLICDomain as InterruptControllerDomain;
use log::warn;
use shared_heap::DVec;

use super::{require_mmio_range_or_einval, try_virtio_mmio_range_or_skip};
use crate::{create_domain, domain_proxy::*, mmio_bus, platform_bus, register_domain};

pub(super) fn init_device() -> AlienResult<Arc<dyn InterruptControllerDomain>> {
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
    let plic_address = require_mmio_range_or_einval("riscv64", "plic", plic_device.locator())?;
    let plic_info = PlicInfo {
        device_info: plic_address,
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
        let irq = device.irq();
        match device.name() {
            "rtc" => {
                if let Some(compatible) = device.compatible() {
                    if compatible != "google,goldfish-rtc" {
                        println_color!(31, "unknown rtc device: {}", compatible);
                        continue;
                    }
                }
                let rtc_range = require_mmio_range_or_einval("riscv64", "rtc", device.locator())?;
                let (rtc, domain_file_info) =
                    create_domain!(RtcDomainProxy, DomainTypeRaw::RtcDomain, "goldfish")?;
                rtc.init_by_box(Box::new(rtc_range))?;
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

                let uart_range = require_mmio_range_or_einval("riscv64", "uart", device.locator())?;

                uart.init_by_box(Box::new(uart_range))?;
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
                let sdcard_range =
                    require_mmio_range_or_einval("riscv64", "sdcard", device.locator())?;
                let (sdcard, domain_file_info) =
                    create_domain!(BlkDomainProxy, DomainTypeRaw::BlkDeviceDomain, "vf2_sd")?;
                sdcard.init_by_box(Box::new(VirtioInitInfo::mmio(sdcard_range, irq)))?;
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
        let Some(mmio_range) = try_virtio_mmio_range_or_skip(
            "riscv64",
            device.device_type(),
            device.mmio_range(),
            device.locator(),
        )?
        else {
            continue;
        };
        match device.device_type() {
            VirtioMmioDeviceType::Network => {
                let (net_driver, domain_file_info) = create_domain!(
                    NetDeviceDomainProxy,
                    DomainTypeRaw::NetDeviceDomain,
                    "virtio_net"
                )?;
                net_driver.init_by_box(Box::new(VirtioInitInfo::mmio(mmio_range, device.irq())))?;
                let net_domain = DomainType::NetDeviceDomain(net_driver.clone());
                register_domain!(
                    "virtio_net",
                    domain_file_info.clone(),
                    net_domain.clone(),
                    false
                );
                register_domain!("nic", domain_file_info, net_domain, false);
            }
            VirtioMmioDeviceType::Block => {
                let (blk_driver, domain_file_info) =
                    create_domain!(BlkDomainProxy, DomainTypeRaw::BlkDeviceDomain, "virtio_blk")?;
                blk_driver.init_by_box(Box::new(VirtioInitInfo::mmio(mmio_range, device.irq())))?;
                let blk_domain = DomainType::BlkDeviceDomain(blk_driver.clone());
                register_domain!(
                    "virtio_block",
                    domain_file_info.clone(),
                    blk_domain.clone(),
                    false
                );
                register_domain!("block", domain_file_info, blk_domain, false);
            }
            VirtioMmioDeviceType::Input => {
                let (input_driver, domain_file_info) =
                    create_domain!(InputDomainProxy, DomainTypeRaw::InputDomain, "virtio_input")?;
                input_driver
                    .init_by_box(Box::new(VirtioInitInfo::mmio(mmio_range, device.irq())))?;
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
                gpu_driver.init_by_box(Box::new(VirtioInitInfo::mmio(mmio_range, device.irq())))?;
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

    let (null_device, domain_file_info) = create_domain!(
        EmptyDeviceDomainProxy,
        DomainTypeRaw::EmptyDeviceDomain,
        "null"
    )?;
    null_device.init_by_box(Box::new(()))?;
    register_domain!(
        "null",
        domain_file_info,
        DomainType::EmptyDeviceDomain(null_device),
        true
    );
    let (random_device, domain_file_info) = create_domain!(
        EmptyDeviceDomainProxy,
        DomainTypeRaw::EmptyDeviceDomain,
        "random"
    )?;
    random_device.init_by_box(Box::new(()))?;
    register_domain!(
        "random",
        domain_file_info,
        DomainType::EmptyDeviceDomain(random_device),
        true
    );

    Ok(plic)
}
