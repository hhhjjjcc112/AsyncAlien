use core::fmt::{Arguments, Result, Write};

use spin::Mutex;

use crate::console_putchar;
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        let cpu_id = $crate::current_cpu_id();
        $crate::console::__print(format_args!("[{}] {}", cpu_id, format_args!($($arg)*)))
    };
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($fmt:expr) => ($crate::print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(
        concat!($fmt, "\n"), $($arg)*));
}

/// 彩色输出。
///
/// 第一个参数为 ANSI 颜色码：
/// - 30: 黑
/// - 31: 红
/// - 32: 绿
/// - 33: 黄
/// - 34: 蓝
/// - 35: 洋红
/// - 36: 青
/// - 37: 白
///
/// # 示例
/// ```rust
/// use platform::println_color;
/// println_color!(31, "这是一条红色日志");
/// ```
#[macro_export]
macro_rules! println_color {
    ($color:expr, $fmt:expr) => {
        $crate::print!(concat!("\x1b[", $color, "m", $fmt, "\x1b[0m\n"));
    };
    ($color:expr, $fmt:expr, $($arg:tt)*) => {
        $crate::print!(concat!("\x1b[", $color, "m", $fmt, "\x1b[0m\n"), $($arg)*);
    };
}

#[macro_export]
macro_rules! iprint {
    ($($arg:tt)*) => {
        $crate::console::__print(format_args!("{}", format_args!($($arg)*)))
    };
}

#[macro_export]
macro_rules! early_print {
    ($($arg:tt)*) => {
        #[cfg(target_arch = "x86_64")]
        {
            let cpu_id = $crate::cpu_id_early();
            $crate::console::__print(format_args!("[{}] {}", cpu_id, format_args!($($arg)*)))
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            $crate::print!($($arg)*)
        }
    };
}

#[macro_export]
macro_rules! early_println {
    () => ($crate::early_print!("\n"));
    ($fmt:expr) => ($crate::early_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::early_print!(
        concat!($fmt, "\n"), $($arg)*));
}


pub struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> Result {
        s.as_bytes().iter().for_each(|x| {
            console_putchar(*x);
        });
        Ok(())
    }
}

static K_STDOUT: Mutex<Stdout> = Mutex::new(Stdout);

pub fn __print(args: Arguments) {
    K_STDOUT.lock().write_fmt(args).unwrap();
}
