//! x86-64 specific page table structures.
//!
//! x86-64 uses 4-level paging with the following hierarchy:
//! - PML4 (Page Map Level 4) - 512 entries, each covers 512GB
//! - PDPT (Page Directory Pointer Table) - 512 entries, each covers 1GB
//! - PD (Page Directory) - 512 entries, each covers 2MB
//! - PT (Page Table) - 512 entries, each covers 4KB
//!
//! Optional 5-level paging (LA57) adds PML5, extending VA to 57 bits.

use crate::{page_table_entry::x86_64::X64PTE, PageTable64, PagingMetaData};

/// Metadata for x86-64 4-level paging (PML4).
///
/// - 48-bit virtual addresses (canonical form)
/// - 52-bit physical addresses (maximum)
/// - 4 levels: PML4 → PDPT → PD → PT
#[derive(Clone, Copy)]
pub struct X64PML4MetaData;

impl const PagingMetaData for X64PML4MetaData {
    const LEVELS: usize = 4;
    const PA_MAX_BITS: usize = 52;
    const VA_MAX_BITS: usize = 48;
    
    #[inline]
    fn vaddr_is_valid(vaddr: usize) -> bool {
        // x86-64 canonical address: bits 47..63 must all be same as bit 47
        let top_bits = vaddr >> 47;
        top_bits == 0 || top_bits == 0x1FFFF
    }
}

/// Metadata for x86-64 5-level paging (LA57/PML5).
///
/// - 57-bit virtual addresses (canonical form)
/// - 52-bit physical addresses (maximum)
/// - 5 levels: PML5 → PML4 → PDPT → PD → PT
#[derive(Clone, Copy)]
pub struct X64PML5MetaData;

impl const PagingMetaData for X64PML5MetaData {
    const LEVELS: usize = 5;
    const PA_MAX_BITS: usize = 52;
    const VA_MAX_BITS: usize = 57;
    
    #[inline]
    fn vaddr_is_valid(vaddr: usize) -> bool {
        // LA57 canonical address: bits 56..63 must all be same as bit 56
        let top_bits = vaddr >> 56;
        top_bits == 0 || top_bits == 0xFF
    }
}

/// x86-64 4-level page table (PML4).
///
/// This is the standard paging mode for x86-64 processors.
pub type X64PageTable<I> = PageTable64<X64PML4MetaData, X64PTE, I>;

/// x86-64 5-level page table (PML5/LA57).
///
/// Available on newer Intel processors with LA57 support.
pub type X64PageTable5<I> = PageTable64<X64PML5MetaData, X64PTE, I>;
