mod device;

use alloc::vec::Vec;

use device::{PciBus, PciCommonDevice, PciDeviceId};
use ksync::Mutex;

use crate::bus::CommonDeviceInfo;

static PCI_BUS: Mutex<PciBus> = Mutex::new(PciBus::new());

const PCI_CFG_ADDR: u16 = 0x0cf8;
const PCI_CFG_DATA: u16 = 0x0cfc;

#[inline]
fn pci_cfg_address(bus: u8, device: u8, function: u8, offset: u8) -> u32 {
	(1u32 << 31)
		| ((bus as u32) << 16)
		| ((device as u32) << 11)
		| ((function as u32) << 8)
		| ((offset as u32) & 0xfc)
}

#[inline]
fn pci_cfg_read32(bus: u8, device: u8, function: u8, offset: u8) -> u32 {
	unsafe {
		x86::io::outl(PCI_CFG_ADDR, pci_cfg_address(bus, device, function, offset));
		x86::io::inl(PCI_CFG_DATA)
	}
}

#[inline]
fn probe_function(bus: u8, device: u8, function: u8) -> Option<PciDeviceId> {
	let id = pci_cfg_read32(bus, device, function, 0x00);
	let vendor_id = (id & 0xffff) as u16;
	if vendor_id == 0xffff {
		return None;
	}

	let device_id = ((id >> 16) & 0xffff) as u16;
	let class_reg = pci_cfg_read32(bus, device, function, 0x08);
	let header = pci_cfg_read32(bus, device, function, 0x0c);

	Some(PciDeviceId {
		bus,
		device,
		function,
		vendor_id,
		device_id,
		class_code: ((class_reg >> 24) & 0xff) as u8,
		subclass: ((class_reg >> 16) & 0xff) as u8,
		prog_if: ((class_reg >> 8) & 0xff) as u8,
		header_type: ((header >> 16) & 0xff) as u8,
	})
}

pub fn pci_init(pci_info: CommonDeviceInfo) {
	let mut discovered = 0usize;
	let mut bus_guard = PCI_BUS.lock();

	// x86 先用配置端口扫描设备，后续可再按 ECAM 优化。
	for bus in 0u8..=u8::MAX {
		for dev in 0u8..32 {
			let Some(first_fn) = probe_function(bus, dev, 0) else {
				continue;
			};

			log::info!(
				"PCI {:02x}:{:02x}.0 {:04x}:{:04x} class={:02x}:{:02x}:{:02x}",
				first_fn.bus,
				first_fn.device,
				first_fn.vendor_id,
				first_fn.device_id,
				first_fn.class_code,
				first_fn.subclass,
				first_fn.prog_if
			);

			bus_guard.register_common_device(PciCommonDevice::new(first_fn, pci_info.clone()));
			discovered += 1;

			if first_fn.header_type & 0x80 == 0 {
				continue;
			}

			for function in 1u8..8 {
				if let Some(other_fn) = probe_function(bus, dev, function) {
					log::info!(
						"PCI {:02x}:{:02x}.{} {:04x}:{:04x} class={:02x}:{:02x}:{:02x}",
						other_fn.bus,
						other_fn.device,
						other_fn.function,
						other_fn.vendor_id,
						other_fn.device_id,
						other_fn.class_code,
						other_fn.subclass,
						other_fn.prog_if
					);
					bus_guard.register_common_device(PciCommonDevice::new(other_fn, pci_info.clone()));
					discovered += 1;
				}
			}
		}
	}

	log::info!("PCI scan done, discovered {} function(s)", discovered);
}

const VIRTIO_VENDOR_ID: u16 = 0x1af4;
const VIRTIO_TRANSITIONAL_BLOCK_ID: u16 = 0x1001;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VirtioPciKind {
	Block,
	Net,
	Input,
	Gpu,
	Other,
}

fn is_virtio_block(dev: PciDeviceId) -> bool {
	if dev.vendor_id != VIRTIO_VENDOR_ID {
		return false;
	}

	// Transitional virtio-blk: 0x1001; modern virtio: 0x1040 + type(2)
	dev.device_id == VIRTIO_TRANSITIONAL_BLOCK_ID || dev.device_id == 0x1042
}

fn parse_virtio_kind(dev: PciDeviceId) -> Option<VirtioPciKind> {
	if dev.vendor_id != VIRTIO_VENDOR_ID {
		return None;
	}

	let kind = match dev.device_id {
		0x1000 | 0x1041 => VirtioPciKind::Net,
		0x1001 | 0x1042 => VirtioPciKind::Block,
		0x1050 => VirtioPciKind::Gpu,
		0x1052 => VirtioPciKind::Input,
		_ => VirtioPciKind::Other,
	};
	Some(kind)
}

pub fn pci_devices() -> Vec<PciCommonDevice> {
	PCI_BUS
		.lock()
		.common_devices()
		.iter()
		.cloned()
		.collect()
}

pub fn virtio_pci_devices() -> Vec<(PciDeviceId, VirtioPciKind)> {
	PCI_BUS
		.lock()
		.common_devices()
		.iter()
		.filter_map(|dev| parse_virtio_kind(dev.dev_id()).map(|kind| (dev.dev_id(), kind)))
		.collect()
}

pub fn has_virtio_block() -> bool {
	PCI_BUS
		.lock()
		.common_devices()
		.iter()
		.any(|dev| is_virtio_block(dev.dev_id()))
}

pub fn pci_config_space() -> Option<core::ops::Range<usize>> {
	PCI_BUS
		.lock()
		.common_devices()
		.front()
		.map(|dev| dev.info().address_range.start.as_usize()..dev.info().address_range.end.as_usize())
}
