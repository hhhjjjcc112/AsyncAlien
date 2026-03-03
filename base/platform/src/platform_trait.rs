//! Platform trait compatibility module
//!
//! This module provides backward compatibility traits that are auto-implemented
//! for types implementing the new MachineInfo trait.

use crate::traits::MachineInfo;

/// Legacy platform information trait (backward compatibility)
///
/// This trait is auto-implemented for all types that implement `MachineInfo`.
pub trait PlatformInfo {
    fn memory_start(&self) -> usize;
    fn memory_size(&self) -> usize;
    fn cpu_count(&self) -> usize;
}

impl<T: MachineInfo> PlatformInfo for T {
    fn memory_start(&self) -> usize {
        MachineInfo::memory_start(self)
    }
    
    fn memory_size(&self) -> usize {
        MachineInfo::memory_size(self)
    }
    
    fn cpu_count(&self) -> usize {
        MachineInfo::cpu_count(self)
    }
}
