use core::sync::atomic::{AtomicUsize, AtomicBool, Ordering};

use alloc::boxed::Box;
use alloc::vec::Vec;
use basic::{config::FRAME_SIZE, time};
use basic::constants::io::{MMapFlags, MMapType, ProtFlags};
use mem::{FrameTracker, MappingFlags, alloc_frame_trackers_no_dealloc};
use shared_heap::DVec;
use vdso_api::MemIf;

// 记录内核侧 vDSO 映射的 vVAR 区域的基地址，以便刷新时间快照时写入数据。
static VVAR_BASE: AtomicUsize = AtomicUsize::new(0);
// 在内核初始化 vDSO 阶段设为 true，供 ppage_alloc 判定是否为 init 期间的分配
static VDSO_LOADED: AtomicBool = AtomicBool::new(false);

fn align_up(value: usize, align: usize) -> usize {
    (value + align - 1) & !(align - 1)
}

/// 内核侧 vDSO 初始化：在内核地址空间映射 vDSO/vVAR 并初始化 vtable。
pub fn init_vdso() {
    // 在 init 阶段设置标志，这样在 ppage_alloc 中可标记为不需要释放的共享页
    VDSO_LOADED.store(true, Ordering::SeqCst);
    // 调用 vdso_api：加载到内核地址空间并初始化 vDSO vtable。
    let vdso_base = vdso_api::map_so(mem::kernel_page_table_token()) as usize;
    unsafe { vdso_api::init_vdso_vtable(vdso_base as u64) };
    VDSO_LOADED.store(false, Ordering::SeqCst);

    let vvar_size = align_up(core::mem::size_of::<vdso_api::VvarData>(), FRAME_SIZE);
    let vvar_base = vdso_base.wrapping_sub(vvar_size);
    VVAR_BASE.store(vvar_base, Ordering::Release);

    refresh_time_snapshot();
}

/// 刷新内核侧保存的 vDSO 共享时间快照。
pub fn refresh_time_snapshot() {
    let vvar_base = VVAR_BASE.load(Ordering::Acquire);
    if vvar_base == 0 {
        return;
    }

    let realtime_ns = time::wall_time_nanos() as usize;
    let monotonic_ns = time::monotonic_time_nanos() as usize;
    
    // 直接将地址按 vdso_api::VvarData 的布局写入，避免在内核重复定义布局。
    let data = unsafe { &mut *(vvar_base as *mut vdso_api::VvarData) };
    let seq = data.seq.wrapping_add(1) | 1;
    data.seq = seq;
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::Release);
    data.realtime_ns = realtime_ns;
    data.monotonic_ns = monotonic_ns;
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::Release);
    data.seq = seq.wrapping_add(1);
}

struct KernelVdsoMem;

fn to_mem_flags(flags: vdso_api::MappingFlags) -> MappingFlags {
    MappingFlags::from_bits_truncate(flags.bits())
}

#[crate_interface::impl_interface]
impl MemIf for KernelVdsoMem {
    fn valloc(vspace: usize, size: usize) -> *mut u8 {
        assert_eq!(size % FRAME_SIZE, 0);
        if vspace == mem::kernel_page_table_token() {
            // 分配虚拟地址，不映射
            let domain_area = mem::reserve_domain_region(size);
            let vaddr = domain_area.as_ptr() as usize;
            
            vaddr as *mut u8
        } else {
            // 在用户地址空间仅预留虚拟区间，不分配物理页、不建映射
            let prot = (ProtFlags::PROT_READ | ProtFlags::PROT_WRITE).bits();
            let flags = MMapFlags::MAP_ANONYMOUS.bits() | (MMapType::Private as u32);
            let res = crate::task_domain!()
                .vdso_reserve_user_vaddr(size, prot, flags)
                .expect("valloc vdso_reserve_user_vaddr 失败");
            res as *mut u8
        }
    }

    fn ppage_alloc(size: usize) -> vdso_api::PhysPagePtr {
        assert_eq!(size % FRAME_SIZE, 0);
        let pages = size / FRAME_SIZE;
        let mut page_descs: Vec<(usize, bool)> = Vec::with_capacity(pages);
        
        // 使用 alloc_frame_trackers_no_dealloc 分配所有物理页，获得 dealloc=false 的代理 trackers
        for _ in 0..pages {
            trace!("Allocating frame for vDSO: size={} bytes", FRAME_SIZE);
            let frame = unsafe { alloc_frame_trackers_no_dealloc(1) };
            let paddr = frame.start();
            // init 阶段分配（内核 init_vdso）标记为 false（kernel 保留所有权）
            // 非 init 阶段分配则标记为 true（需要转移/由 task 释放）
            let transfer_flag = !VDSO_LOADED.load(Ordering::Acquire);
            page_descs.push((paddr, transfer_flag));
        }
        
        // 把 (paddr, transfer_flag) 列表转为 DVec 并返回指针
        let dvec = DVec::from_slice(page_descs.as_slice());
        let ptr = Box::new(dvec);
        Box::into_raw(ptr) as vdso_api::PhysPagePtr
    }

    fn map(
        vspace: usize,
        vaddr: *mut u8,
        ppage: vdso_api::PhysPagePtr,
        size: usize,
        flags: vdso_api::MappingFlags,
    ) {
        assert_eq!(size % FRAME_SIZE, 0);
        // 恢复 DVec<(usize,bool)>：每个 (paddr, transfer_flag) 对
        let page_descs_box = unsafe { Box::from_raw(ppage as *mut DVec<(usize, bool)>) };
        let page_descs = page_descs_box.as_slice();

        if vspace == mem::kernel_page_table_token() {
            // 内核映射：用代理 trackers（dealloc=false）进行内核页表安装
            let mut mapped_frames: Vec<Box<dyn mem::PhysPage>> = Vec::with_capacity(page_descs.len());
            for (paddr, _) in page_descs {
                let start_page = paddr / FRAME_SIZE;
                // 构造代理 tracker（dealloc=false），用于内核映射
                mapped_frames.push(Box::new(FrameTracker::new(start_page, 1, false)));
            }
            mem::map_kernel_pages(vaddr as usize, size, to_mem_flags(flags), mapped_frames);
            // page_descs_box 自然 drop，page_descs 也作为 Box<DVec> 被释放
        } else {
            // 用户地址空间映射：直接传 DVec<(usize,bool)> 给 task domain RPC
            let prot = ProtFlags::from_bits_truncate(flags.bits() as u32).bits();
            
            // 将 Box<DVec> 转回 DVec 以便跨域传递（所有权转移）
            let dvec_page_descs = *page_descs_box;
            
            crate::task_domain!()
                .vdso_map_user_pages(vaddr as usize, size, prot, dvec_page_descs)
                .map_err(|e| {
                    log::error!("vdso_map_user_pages 失败: {:?}", e);
                    e
                }).expect("vdso_map_user_pages 失败");
        }
    }

    fn change_protect(
        vspace: usize,
        vaddr: *mut u8,
        size: usize,
        flags: vdso_api::MappingFlags,
    ) {
        assert_eq!(size % FRAME_SIZE, 0);
        if vspace == mem::kernel_page_table_token() {
            mem::protect_kernel_pages(vaddr as usize, size, to_mem_flags(flags));
        } else {
            let prot = ProtFlags::from_bits_truncate(flags.bits() as u32).bits();
            let _ = crate::task_domain!()
                .do_mprotect(vaddr as usize, size, prot)
                .expect("do_mprotect 失败");
        }
    }

    fn get_kernel_vaddr(vspace: usize, vaddr: *mut u8) -> *mut u8 {
        // 如果是内核页表 token，直接返回原地址；否则通过 task domain 查询物理地址并转为内核虚拟地址。
        if vspace == mem::kernel_page_table_token() {
            return vaddr;
        } else {
            // 向 task 域请求翻译（假定当前上下文下的 task 为目标 task）
            let res = crate::task_domain!().vaddr_to_paddr(vaddr as usize);
            match res {
                Ok(paddr) => {
                    // 将物理地址转为内核可访问虚拟地址
                    let kaddr = <platform::Platform as platform::MemIf>::phys_to_virt(paddr);
                    kaddr as *mut u8
                }
                Err(_) => vaddr,
            }
        }       
    }

    fn ppage_clone(ppage: vdso_api::PhysPagePtr) -> vdso_api::PhysPagePtr {
        // 恢复 Box<DVec<(usize,bool)>>
        let dvec_box = unsafe { Box::from_raw(ppage as *mut DVec<(usize, bool)>) };
        let dvec = dvec_box.as_slice();
        
        // 从原始 DVec 的数据复制出来创建新的 DVec
        let new_dvec = DVec::from_slice(dvec);
        let new_box = Box::new(new_dvec);
        
        // 恢复原始指针（以便后续调用方仍能访问）
        let _ = Box::into_raw(dvec_box);
        
        // 返回新的 Box<DVec> 指针
        Box::into_raw(new_box) as vdso_api::PhysPagePtr
    }
}
