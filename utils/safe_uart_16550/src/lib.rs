#![no_std]

use core::ops::Range;

#[cfg(target_arch = "x86_64")]
use io::port::SafePort;
#[cfg(target_arch = "riscv64")]
use io::region::SafeIORegion;
#[cfg(target_arch = "riscv64")]
use uart_16550::MmioSerialPort;
#[cfg(target_arch = "x86_64")]
use uart_16550::SerialPort;

const UART_REG_WINDOW_SIZE: usize = 8;
const REG_IER: usize = 1;
const REG_LSR: usize = 5;
const IER_RX_ENABLE_BIT: u8 = 1 << 0;
const LSR_DATA_READY_BIT: u8 = 1 << 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UartError {
    InvalidAddressRange,
    UnsupportedTransport,
    IoRegionAccessFailed,
}

pub type Result<T> = core::result::Result<T, UartError>;

#[derive(Debug)]
enum UartInner {
    #[cfg(target_arch = "x86_64")]
    Pio(SerialPort),
    #[cfg(target_arch = "riscv64")]
    Mmio(MmioSerialPort),
}

#[derive(Debug)]
pub struct SafeUart16550 {
    #[cfg(target_arch = "x86_64")]
    port: SafePort,
    #[cfg(target_arch = "riscv64")]
    io_region: SafeIORegion,
    inner: UartInner,
}

impl SafeUart16550 {
    #[cfg(target_arch = "x86_64")]
    pub fn new_pio(address_range: &Range<usize>) -> Result<Self> {
        if !is_valid_range(address_range) || range_size(address_range) < UART_REG_WINDOW_SIZE {
            return Err(UartError::InvalidAddressRange);
        }
        if !is_port_range(address_range) {
            return Err(UartError::UnsupportedTransport);
        }

        let port = SafePort::from_usize_range(address_range.clone())
            .map_err(|_| UartError::InvalidAddressRange)?;
        let base_port = port.port_range().start;
        // 地址由内核枚举并传入，且此处已经明确校验为 PIO 范围。
        let inner = unsafe { SerialPort::new(base_port) };
        Ok(Self {
            port,
            inner: UartInner::Pio(inner),
        })
    }

    #[cfg(target_arch = "riscv64")]
    pub fn new_mmio(address_range: &Range<usize>) -> Result<Self> {
        if !is_valid_range(address_range) || range_size(address_range) < UART_REG_WINDOW_SIZE {
            return Err(UartError::InvalidAddressRange);
        }
        if is_port_range(address_range) {
            return Err(UartError::UnsupportedTransport);
        }

        let io_region = SafeIORegion::from(address_range.clone());
        // 地址由内核枚举并传入，且此处已保证使用 MMIO 语义。
        let inner = unsafe { MmioSerialPort::new(address_range.start) };
        Ok(Self {
            io_region,
            inner: UartInner::Mmio(inner),
        })
    }

    pub fn init(&mut self) {
        match &mut self.inner {
            #[cfg(target_arch = "x86_64")]
            UartInner::Pio(port) => port.init(),
            #[cfg(target_arch = "riscv64")]
            UartInner::Mmio(port) => port.init(),
        }
    }

    pub fn putc(&mut self, ch: u8) {
        match &mut self.inner {
            #[cfg(target_arch = "x86_64")]
            UartInner::Pio(port) => port.send_raw(ch),
            #[cfg(target_arch = "riscv64")]
            UartInner::Mmio(port) => port.send_raw(ch),
        }
    }

    pub fn put_bytes(&mut self, buf: &[u8]) -> usize {
        let mut wrote = 0usize;
        for &ch in buf {
            let ok = match &mut self.inner {
                #[cfg(target_arch = "x86_64")]
                UartInner::Pio(port) => port.try_send_raw(ch).is_ok(),
                #[cfg(target_arch = "riscv64")]
                UartInner::Mmio(port) => port.try_send_raw(ch).is_ok(),
            };
            if !ok {
                break;
            }
            wrote += 1;
        }
        wrote
    }

    pub fn getc_nonblocking(&mut self) -> Option<u8> {
        match &mut self.inner {
            #[cfg(target_arch = "x86_64")]
            UartInner::Pio(port) => port.try_receive().ok(),
            #[cfg(target_arch = "riscv64")]
            UartInner::Mmio(port) => port.try_receive().ok(),
        }
    }

    pub fn have_data_to_get(&self) -> Result<bool> {
        let lsr = self.read_u8(REG_LSR)?;
        Ok((lsr & LSR_DATA_READY_BIT) != 0)
    }

    pub fn enable_receive_interrupt(&self) -> Result<()> {
        let ier = self.read_u8(REG_IER)?;
        self.write_u8(REG_IER, ier | IER_RX_ENABLE_BIT)
    }

    pub fn disable_receive_interrupt(&self) -> Result<()> {
        let ier = self.read_u8(REG_IER)?;
        self.write_u8(REG_IER, ier & !IER_RX_ENABLE_BIT)
    }

    #[cfg(target_arch = "x86_64")]
    fn read_u8(&self, offset: usize) -> Result<u8> {
        self.port
            .read_at::<u8>(offset)
            .map_err(|_| UartError::IoRegionAccessFailed)
    }

    #[cfg(target_arch = "x86_64")]
    fn write_u8(&self, offset: usize, value: u8) -> Result<()> {
        self.port
            .write_at(offset, value)
            .map_err(|_| UartError::IoRegionAccessFailed)
    }

    #[cfg(target_arch = "riscv64")]
    fn read_u8(&self, offset: usize) -> Result<u8> {
        self.io_region
            .read_at::<u8>(offset)
            .map_err(|_| UartError::IoRegionAccessFailed)
    }

    #[cfg(target_arch = "riscv64")]
    fn write_u8(&self, offset: usize, value: u8) -> Result<()> {
        self.io_region
            .write_at(offset, value)
            .map_err(|_| UartError::IoRegionAccessFailed)
    }
}

#[inline]
fn is_valid_range(range: &Range<usize>) -> bool {
    range.start < range.end
}

#[inline]
fn range_size(range: &Range<usize>) -> usize {
    range.end.saturating_sub(range.start)
}

#[inline]
fn is_port_range(range: &Range<usize>) -> bool {
    range.end <= 0x1_0000 && range.start < range.end
}

#[cfg(test)]
mod tests {
    use super::{SafeUart16550, UartError};

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn new_pio_rejects_invalid_range() {
        let err = SafeUart16550::new_pio(&(0x3f8..0x3f8)).unwrap_err();
        assert_eq!(err, UartError::InvalidAddressRange);

        let err = SafeUart16550::new_pio(&(0x3f8..0x3ff)).unwrap_err();
        assert_eq!(err, UartError::InvalidAddressRange);
    }

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn new_pio_rejects_non_pio_transport() {
        let err = SafeUart16550::new_pio(&(0x1000_0000..0x1000_0008)).unwrap_err();
        assert_eq!(err, UartError::UnsupportedTransport);
    }
}
