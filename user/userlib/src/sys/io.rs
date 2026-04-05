use core::fmt::Write;

use core2::io::Read;
use core2::io::{Error, ErrorKind};

use crate::syscall::{sys_read, sys_write};

type Result<T> = core2::io::Result<T>;
pub type Stderr = Stdout;

#[derive(Debug)]
pub struct Stdout;
impl Stdout {
    pub fn new() -> Self {
        Stdout {}
    }
}
impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let len = sys_write(1, s.as_ptr(), s.len());
        if len < 0 {
            return Err(core::fmt::Error);
        }
        Ok(())
    }
}
#[derive(Debug)]
pub struct Stdin;

impl Stdin {
    pub fn new() -> Self {
        Stdin {}
    }
}
impl Read for Stdin {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        loop {
            let len = sys_read(0, buf.as_mut_ptr(), buf.len());
            if len > 0 {
                return Ok(len as usize);
            }
            if len < 0 {
                return Err(Error::new(ErrorKind::Other, "sys_read failed"));
            }
        }
    }
}

impl core2::io::Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let len = sys_write(1, buf.as_ptr(), buf.len());
        if len < 0 {
            return Err(Error::new(ErrorKind::Other, "sys_write failed"));
        }
        Ok(len as usize)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
