use alloc::{boxed::Box, string::ToString, sync::Arc};

use basic::bus::mmio::VirtioMmioDeviceType;
use corelib::AlienResult;
use interface::*;
use log::warn;
use shared_heap::DVec;

use super::{
    InterruptControllerDomain, require_mmio_range_or_einval, try_virtio_mmio_range_or_skip,
};
use crate::{
    bus::DeviceLocator, create_domain, domain_proxy::*, mmio_bus, pci_bus, platform_bus,
    register_domain,
};

fn init_x86_rtc_domain() -> AlienResult<()> {
    // x86 CMOS RTC 使用固定端口，最小实现只提供读时间能力。
    let rtc_range = 0x70usize..0x72usize;
    let (rtc, domain_file_info) =
        create_domain!(RtcDomainProxy, DomainTypeRaw::RtcDomain, "cmos_rtc")?;
    rtc.init_by_box(Box::new(rtc_range))?;
    register_domain!("rtc", domain_file_info, DomainType::RtcDomain(rtc), true);
    Ok(())
}

fn register_platform_owned_empty_device(device_name: &str) -> AlienResult<()> {
    // 这些设备由 base/platform 直接管理，域层只保留名字占位。
    let (empty_dev, domain_file_info) = create_domain!(
        EmptyDeviceDomainProxy,
        DomainTypeRaw::EmptyDeviceDomain,
        "null"
    )?;
    empty_dev.init_by_box(Box::new(()))?;
    register_domain!(
        device_name,
        domain_file_info,
        DomainType::EmptyDeviceDomain(empty_dev),
        true
    );
    Ok(())
}

pub(super) fn init_device() -> AlienResult<Arc<InterruptControllerDomain>> {
    let platform_bus = platform_bus!();
    let mut has_nic = false;
    let mut has_block = false;
    let mut has_gpu_alias = false;
    let mut input_alias_count = 0usize;

    let (apic, domain_file_info) =
        create_domain!(APICDomainProxy, DomainTypeRaw::APICDomain, "apic")?;
    apic.init_by_box(Box::new(()))?;
    register_domain!(
        "apic",
        domain_file_info,
        DomainType::APICDomain(apic.clone()),
        true
    );

    for device in platform_bus.common_devices().iter() {
        let irq = device.irq();

        match device.name() {
            "uart" => {
                let compatible = device.compatible().ok_or_else(|| {
                    log::error!("uart device missing compatible property");
                    crate::error::AlienError::EINVAL
                })?;
                let (uart, domain_file_info) = match compatible {
                    "ns16550a" => {
                        create_domain!(UartDomainProxy, DomainTypeRaw::UartDomain, "uart16550")?
                    }
                    "snps,dw-apb-uart" => {
                        create_domain!(UartDomainProxy, DomainTypeRaw::UartDomain, "uart8250")?
                    }
                    _ => {
                        log::error!("unknown uart device: {}", compatible);
                        return Err(crate::error::AlienError::EINVAL);
                    }
                };

                let uart_range = match device.locator() {
                    DeviceLocator::Pio(range) => usize::from(range.start)..usize::from(range.end),
                    other => {
                        log::error!("x86_64 uart locator must be PIO, got {:?}", other);
                        return Err(crate::error::AlienError::EINVAL);
                    }
                };

                uart.init_by_box(Box::new(uart_range))?;
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
                init_x86_rtc_domain()?;
            }
            "ramdisk" => {
                let ramdisk_range =
                    require_mmio_range_or_einval("x86_64", "ramdisk", device.locator())?;
                let (ramdisk, domain_file_info) =
                    create_domain!(BlkDomainProxy, DomainTypeRaw::BlkDeviceDomain, "mem_block")?;
                ramdisk.init_by_box(Box::new(VirtioInitInfo::mmio(ramdisk_range, irq)))?;
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
            "loopback" => {
                let loopback_range =
                    require_mmio_range_or_einval("x86_64", "loopback", device.locator())?;
                let (loopback, domain_file_info) = create_domain!(
                    NetDeviceDomainProxy,
                    DomainTypeRaw::NetDeviceDomain,
                    "loopback"
                )?;
                loopback.init_by_box(Box::new(VirtioInitInfo::mmio(loopback_range, irq)))?;
                register_domain!(
                    "loopback",
                    domain_file_info,
                    DomainType::NetDeviceDomain(loopback),
                    false
                );
            }
            "pci_ecam" => {
                // x86_64 的 virtio PCI 统一在平台设备遍历后处理，这里保留分支用于兼容旧日志路径。
            }
            "local_apic" | "io_apic" | "hpet" => {
                register_platform_owned_empty_device(device.name())?;
            }
            _ => {
                warn!("unknown device: {}", device.name());
            }
        }
    }

    // x86_64 不依赖 platform_bus 是否暴露 pci_ecam，统一按 PCI endpoint 绑定 virtio 域。
    let (blk_ep, net_ep, input_eps, gpu_ep) = {
        let bus = pci_bus!().read();
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
        let pci_irq = ep.interrupt_line().map(|irq| irq as u32);
        let legacy_io = ep.legacy_io_range();
        let modern = ep.virtio_modern_info();
        let use_modern = !ep.is_transitional_virtio();
        if !use_modern && modern.is_some() {
            log::debug!(
                "virtio-blk @ {:04x}:{:02x}:{:02x}.{} is transitional, prefer legacy transport",
                bdf.segment(),
                bdf.bus(),
                bdf.device(),
                bdf.function()
            );
        }
        if use_modern {
            if let Some(modern_info) = modern.as_ref() {
                mem::map_device_phys_range(modern_info.common.clone());
                mem::map_device_phys_range(modern_info.notify.clone());
                mem::map_device_phys_range(modern_info.isr.clone());
                mem::map_device_phys_range(modern_info.device.clone());
            }
        }
        if legacy_io.is_some() || (use_modern && modern.is_some()) {
            let init_info = VirtioInitInfo::pci(
                bdf.segment(),
                bdf.bus(),
                bdf.device(),
                bdf.function(),
                pci_irq,
                legacy_io,
            )
            .with_modern_pci(
                if use_modern {
                    modern.as_ref().map(|x| x.common.clone())
                } else {
                    None
                },
                if use_modern {
                    modern.as_ref().map(|x| x.notify.clone())
                } else {
                    None
                },
                if use_modern {
                    modern.as_ref().map(|x| x.notify_off_multiplier)
                } else {
                    None
                },
                if use_modern {
                    modern.as_ref().map(|x| x.isr.clone())
                } else {
                    None
                },
                if use_modern {
                    modern.as_ref().map(|x| x.device.clone())
                } else {
                    None
                },
            );
            let (blk_driver, domain_file_info) =
                create_domain!(BlkDomainProxy, DomainTypeRaw::BlkDeviceDomain, "virtio_blk")?;
            blk_driver.init_by_box(Box::new(init_info))?;
            println!(
                "virtio-pci block @ {:04x}:{:02x}:{:02x}.{}",
                bdf.segment(),
                bdf.bus(),
                bdf.device(),
                bdf.function()
            );
            let blk_domain = DomainType::BlkDeviceDomain(blk_driver.clone());
            let virtio_blk_name = register_domain!(
                "virtio_block",
                domain_file_info.clone(),
                blk_domain.clone(),
                false
            );
            register_domain!("block", domain_file_info, blk_domain, false);

            if let Some(irq) = pci_irq {
                log::debug!(
                    "virtio-blk irq {} detected, defer APIC route until trap domain is ready (domain={})",
                    irq,
                    virtio_blk_name
                );
            } else {
                warn!(
                    "virtio-blk @ {:04x}:{:02x}:{:02x}.{} has no irq line",
                    bdf.segment(),
                    bdf.bus(),
                    bdf.device(),
                    bdf.function()
                );
            }

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
            register_domain!("nic", domain_file_info, net_domain, false);
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
            let (input_driver, domain_file_info) =
                create_domain!(InputDomainProxy, DomainTypeRaw::InputDomain, "virtio_input")?;
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

    for device in mmio_bus!().lock().common_devices().iter() {
        let Some(mmio_range) = try_virtio_mmio_range_or_skip(
            "x86_64",
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
                has_nic = true;
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
                has_block = true;
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

    Ok(apic)
}
