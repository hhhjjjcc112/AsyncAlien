mod bus;

use super::{DeviceStatus, DeviceType, Transport};
use crate::{
    PhysAddr,
    error::{VirtIoError, VirtIoResult},
    hal::{Hal, VirtIoDeviceIo},
};
use alloc::boxed::Box;
use bus::*;
use core::fmt::{self, Display, Formatter};

/// The PCI vendor ID for VirtIO devices.
const VIRTIO_VENDOR_ID: u16 = 0x1af4;

/// The offset to add to a VirtIO device ID to get the corresponding PCI device ID.
const PCI_DEVICE_ID_OFFSET: u16 = 0x1040;

const TRANSITIONAL_NETWORK: u16 = 0x1000;
const TRANSITIONAL_BLOCK: u16 = 0x1001;
const TRANSITIONAL_MEMORY_BALLOONING: u16 = 0x1002;
const TRANSITIONAL_CONSOLE: u16 = 0x1003;
const TRANSITIONAL_SCSI_HOST: u16 = 0x1004;
const TRANSITIONAL_ENTROPY_SOURCE: u16 = 0x1005;
const TRANSITIONAL_9P_TRANSPORT: u16 = 0x1009;

fn map_device_type(pci_device_id: u16) -> DeviceType {
    match pci_device_id {
        TRANSITIONAL_NETWORK => DeviceType::Network,
        TRANSITIONAL_BLOCK => DeviceType::Block,
        TRANSITIONAL_MEMORY_BALLOONING => DeviceType::MemoryBalloon,
        TRANSITIONAL_CONSOLE => DeviceType::Console,
        TRANSITIONAL_SCSI_HOST => DeviceType::ScsiHost,
        TRANSITIONAL_ENTROPY_SOURCE => DeviceType::EntropySource,
        TRANSITIONAL_9P_TRANSPORT => DeviceType::_9P,
        id if id >= PCI_DEVICE_ID_OFFSET => DeviceType::from(id - PCI_DEVICE_ID_OFFSET),
        _ => DeviceType::Invalid,
    }
}

/// Returns the type of VirtIO device to which the given PCI vendor and device ID corresponds.
pub fn virtio_device_type(device_function_info: &DeviceFunctionInfo) -> Option<DeviceType> {
    if device_function_info.vendor_id == VIRTIO_VENDOR_ID {
        let ty = map_device_type(device_function_info.device_id);
        if ty != DeviceType::Invalid {
            return Some(ty);
        }
    }
    None
}

/// PCI transport for VirtIO.
///
/// 当前仓库临时禁用该实现，仅保留接口以兼容上层调用。
#[derive(Debug)]
pub struct PciTransport {
    io_region: Box<dyn VirtIoDeviceIo>,
}

impl PciTransport {
    /// Construct a new PCI VirtIO transport.
    ///
    /// 当前返回 `Disabled`，用于显式标识该路径被禁用。
    pub fn new<H: Hal>(
        _root: &mut PciRoot,
        _device_function: DeviceFunction,
    ) -> Result<Self, VirtioPciError> {
        let _ = core::marker::PhantomData::<H>;
        Err(VirtioPciError::Disabled)
    }
}

impl Transport for PciTransport {
    fn device_type(&self) -> VirtIoResult<DeviceType> {
        Err(VirtIoError::Unsupported)
    }

    fn read_device_features(&mut self) -> VirtIoResult<u64> {
        Err(VirtIoError::Unsupported)
    }

    fn write_driver_features(&mut self, _driver_features: u64) -> VirtIoResult<()> {
        Err(VirtIoError::Unsupported)
    }

    fn max_queue_size(&mut self, _queue: u16) -> VirtIoResult<u32> {
        Err(VirtIoError::Unsupported)
    }

    fn notify(&mut self, _queue: u16) -> VirtIoResult<()> {
        Err(VirtIoError::Unsupported)
    }

    fn get_status(&self) -> VirtIoResult<DeviceStatus> {
        Err(VirtIoError::Unsupported)
    }

    fn set_status(&mut self, _status: DeviceStatus) -> VirtIoResult<()> {
        Err(VirtIoError::Unsupported)
    }

    fn set_guest_page_size(&mut self, _guest_page_size: u32) -> VirtIoResult<()> {
        Ok(())
    }

    fn requires_legacy_layout(&self) -> bool {
        false
    }

    fn queue_set(
        &mut self,
        _queue: u16,
        _size: u32,
        _descriptors: PhysAddr,
        _driver_area: PhysAddr,
        _device_area: PhysAddr,
    ) -> VirtIoResult<()> {
        Err(VirtIoError::Unsupported)
    }

    fn queue_unset(&mut self, _queue: u16) -> VirtIoResult<()> {
        Ok(())
    }

    fn queue_used(&mut self, _queue: u16) -> VirtIoResult<bool> {
        Err(VirtIoError::Unsupported)
    }

    fn ack_interrupt(&mut self) -> VirtIoResult<bool> {
        Err(VirtIoError::Unsupported)
    }

    fn io_region(&self) -> &dyn VirtIoDeviceIo {
        self.io_region.as_ref()
    }
}

/// An error encountered initialising a VirtIO PCI transport.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VirtioPciError {
    /// PCI transport is temporarily disabled in this workspace.
    Disabled,
    /// A generic PCI error.
    Pci(PciError),
}

impl Display for VirtioPciError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Disabled => write!(f, "PCI transport is disabled"),
            Self::Pci(pci_error) => pci_error.fmt(f),
        }
    }
}

impl From<PciError> for VirtioPciError {
    fn from(error: PciError) -> Self {
        Self::Pci(error)
    }
}
