//! Console I/O interface
//!
//! Provides character-level console input/output abstraction.

use core::fmt::{Arguments, Write};

/// Console I/O trait
///
/// Platform implementations provide basic character I/O through serial port,
/// UART, or other console devices.
pub trait ConsoleIf {
    /// Write a single byte to the console
    fn putchar(ch: u8);

    /// Read a single byte from the console (non-blocking)
    /// Returns None if no data available
    fn getchar() -> Option<u8>;

    /// Write a string slice to the console
    fn write_str(s: &str) {
        for b in s.bytes() {
            Self::putchar(b);
        }
    }

    /// Write a byte slice to the console
    fn write_bytes(bytes: &[u8]) {
        for &b in bytes {
            Self::putchar(b);
        }
    }

    /// Read bytes from the console into buffer (non-blocking)
    /// Returns number of bytes actually read
    fn read_bytes(buf: &mut [u8]) -> usize {
        let mut count = 0;
        for slot in buf.iter_mut() {
            if let Some(ch) = Self::getchar() {
                *slot = ch;
                count += 1;
            } else {
                break;
            }
        }
        count
    }
}
