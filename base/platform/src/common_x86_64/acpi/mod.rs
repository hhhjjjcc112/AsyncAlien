extern crate alloc;

use spin::Once;

mod acpi_handler;
mod aml_handler;
mod dynamic;
mod info;
mod static_info;
mod support;

pub use self::info::{AcpiDeviceEntry, AcpiDeviceInfo, AcpiDeviceList};

const AML_MUTEX_MAX: usize = 64;

#[inline]
fn use_static_acpi() -> bool {
    crate::qemu_x86_64::config::STATIC_ACPI
}

pub fn device_list() -> AcpiDeviceList {
    device_info().devices.clone()
}

static ACPI_INFO: Once<AcpiDeviceInfo> = Once::new();

pub fn init() {
    ACPI_INFO.call_once(detect_acpi_info);
}

pub fn device_info() -> AcpiDeviceInfo {
    init();
    ACPI_INFO.get().cloned().unwrap_or_default()
}

fn detect_acpi_info() -> AcpiDeviceInfo {
    if use_static_acpi() {
        return static_info::detect_static_acpi_info();
    }
    dynamic::detect_acpi_info()
}