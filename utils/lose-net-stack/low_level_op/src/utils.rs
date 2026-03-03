use core::mem::size_of;
pub struct BufIter<'a> {
    buf: &'a mut [u8],
    pos: usize,
}
impl<'a> BufIter<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self { buf, pos: 0 }
    }
    /// get the ref array the current pointer point to.
    pub fn get_curr_arr(self) -> &'a [u8] {
        &self.buf[self.pos..]
    }
    /// get the mut ref array the current pointer point to.
    pub fn get_curr_arr_mut(self) -> &'a mut [u8] {
        &mut self.buf[self.pos..]
    }

    /// get a ref from the current ptr position
    pub fn next<T>(&mut self) -> Option<&'a T> {
        if self.pos + size_of::<T>() <= self.buf.len() {
            let v = &self.buf[self.pos..] as *const [u8] as *const T;
            self.pos += size_of::<T>();
            unsafe { v.as_ref() }
        } else {
            None
        }
    }

    /// get a mutable ref from the current ptr position
    pub fn next_mut<T>(&mut self) -> Option<&'a mut T> {
        if self.pos + size_of::<T>() <= self.buf.len() {
            let v = &self.buf[self.pos..] as *const [u8] as *mut T;
            self.pos += size_of::<T>();
            unsafe { v.as_mut() }
        } else {
            None
        }
    }
}

// check sum function
pub fn check_sum(addr: *mut u8, len: u32, sum: u32) -> u16 {
    let mut sum: u32 = sum;
    let mut nleft = len;
    let mut w = addr as *const u16;

    while nleft > 1 {
        sum += unsafe { *w as u32 };
        w = (w as usize + 2) as *mut u16;
        nleft -= 2;

        if sum > 0xffff {
            sum = (sum & 0xFFFF) + (sum >> 16);
            sum = sum + (sum >> 16);
        }
    }

    if nleft == 1 {
        sum += unsafe { *(w as *const u8) as u32 };
    }

    sum = (sum & 0xFFFF) + (sum >> 16);
    sum = sum + (sum >> 16);

    let answer: u16 = !sum as u16;

    answer
}
