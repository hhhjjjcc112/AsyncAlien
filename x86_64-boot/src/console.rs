use core::fmt::{self, Write};

use kspin::SpinNoIrq;
use uart_16550::SerialPort;

const CONSOLE_PORT: u16 = 0x3F8;
pub struct Console(SpinNoIrq<SerialPort>);
static CONSOLE: Console = Console(SpinNoIrq::new(
    unsafe { SerialPort::new(CONSOLE_PORT) }));

pub fn init_console() {
    let mut console = CONSOLE.0.lock();
    console.init();
}

pub fn print(fmt: fmt::Arguments) {
    CONSOLE.0.lock().write_fmt(fmt).unwrap();
}

impl Write for Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let mut serial = self.0.lock();
        for byte in s.bytes() {
            serial.send(byte);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    };
    () => {
        $crate::console::print(format_args!(""));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    };
    () => {
        $crate::console::print(format_args!("\n"));
    }
}