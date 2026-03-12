//! 控制台字符 I/O 接口。

use core::fmt::{Arguments, Write};

/// 控制台 I/O 抽象。
pub trait ConsoleIf {
    /// 输出单字节。
    fn putchar(ch: u8);

    /// 非阻塞读单字节，无数据返回 `None`。
    fn getchar() -> Option<u8>;

    /// 输出字符串。
    fn write_str(s: &str) {
        for b in s.bytes() {
            Self::putchar(b);
        }
    }

    /// 输出字节切片。
    fn write_bytes(bytes: &[u8]) {
        for &b in bytes {
            Self::putchar(b);
        }
    }

    /// 非阻塞读取到缓冲区，返回实际读到的字节数。
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
