use config::FRAME_SIZE;
use ksync::Mutex;
use platform::{println, MemIf, Platform};
use ptable::PhysPage;

use crate::frame::FrameTracker;

pub static INITRD_DATA: Mutex<Option<InitrdData>> = Mutex::new(None);

fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xffff_ffffu32;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xedb8_8320u32 & mask);
        }
    }
    !crc
}

/// 重定位后的 initrd 数据。
pub struct InitrdData {
    frames: FrameTracker,
    size: usize,
}

impl InitrdData {
    pub fn as_slice(&self) -> &[u8] {
        self.frames.as_bytes()[..self.size].as_ref()
    }
}

pub(super) fn relocate_removable_data() {
    let info = platform::platform_machine_info();
    if info.initrd.is_some() {
        let start = info.initrd.as_ref().unwrap().start;
        let end = info.initrd.as_ref().unwrap().end;
        let size = end - start;
        let np = (size + FRAME_SIZE - 1) / FRAME_SIZE;
        let frame_tracker = crate::alloc_frame_trackers(np);
        let src_vaddr = Platform::phys_to_virt(start);
        // 将 boot_info 给出的 initrd 搬到可管理页帧中。
        unsafe {
            core::ptr::copy_nonoverlapping(
                src_vaddr as *const u8,
                frame_tracker.phys_addr().as_usize() as _,
                size,
            );
        }
        let src_bytes = unsafe { core::slice::from_raw_parts(src_vaddr as *const u8, size) };
        let dst_bytes = &frame_tracker.as_bytes()[..size];
        let src_head = &src_bytes[..size.min(4)];
        let dst_head = &dst_bytes[..size.min(4)];
        println!(
            "initrd copy: src={:#x}, size={}, head={:x?}->{:x?}",
            src_vaddr, size, src_head, dst_head
        );
        println!(
            "initrd copy crc32: src={:#x}, dst={:#x}",
            crc32(src_bytes),
            crc32(dst_bytes)
        );
        println!(
            "Relocate initrd data to {:#x}",
            frame_tracker.phys_addr().as_usize()
        );
        let mut guard = INITRD_DATA.lock();
        let data = InitrdData {
            frames: frame_tracker,
            size,
        };
        *guard = Some(data);
    }
}

impl Drop for InitrdData {
    fn drop(&mut self) {
        println!("Drop initrd data");
    }
}
