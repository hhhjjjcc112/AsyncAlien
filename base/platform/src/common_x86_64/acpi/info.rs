const DEFAULT_LAPIC_BASE: usize = 0xfee0_0000;
const DEFAULT_IOAPIC_BASE: usize = 0xfec0_0000;

#[derive(Clone, Debug)]
pub struct AcpiDeviceInfo {
    pub lapic_base: usize,
    pub ioapic_base: usize,
    pub hpet_base: Option<usize>,
    pub devices: AcpiDeviceList,
}

impl Default for AcpiDeviceInfo {
    fn default() -> Self {
        Self {
            lapic_base: DEFAULT_LAPIC_BASE,
            ioapic_base: DEFAULT_IOAPIC_BASE,
            hpet_base: None,
            devices: AcpiDeviceList::default(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AcpiDeviceEntry {
    pub name: &'static str,
    pub base: usize,
    pub size: usize,
}

#[derive(Clone, Debug)]
pub struct AcpiDeviceList {
    pub entries: heapless::Vec<AcpiDeviceEntry, 16>,
}

impl Default for AcpiDeviceList {
    fn default() -> Self {
        Self {
            entries: heapless::Vec::new(),
        }
    }
}