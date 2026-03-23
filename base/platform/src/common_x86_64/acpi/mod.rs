extern crate alloc;

use spin::Once;

use acpi::{rsdp::Rsdp, AcpiTables};

mod acpi_handler;
mod info;
mod static_info;
mod support;

pub use self::info::{AcpiDeviceEntry, AcpiDeviceInfo, AcpiDeviceList};
pub use self::acpi_handler::AcpiHost;

pub fn device_list() -> AcpiDeviceList {
    // 对外只暴露设备列表视图，不暴露内部初始化细节。
    device_info().devices.clone()
}

static ACPI_INFO: Once<AcpiDeviceInfo> = Once::new();
static ACPI_TABLES: Once<Option<AcpiTables<AcpiHost>>> = Once::new();

pub fn init() {
    // ACPI 表只需要初始化一次，后续直接复用缓存结果。
    ACPI_TABLES.call_once(detect_acpi_tables);
    ACPI_INFO.call_once(detect_acpi_info);
}

pub fn device_info() -> AcpiDeviceInfo {
    // 设备信息由静态表和已加载 ACPI 表共同推导，按需懒初始化即可。
    init();
    ACPI_INFO.get().cloned().unwrap_or_default()
}

pub fn tables() -> Option<&'static AcpiTables<AcpiHost>> {
    // bus 侧如果需要做 AML/PCI 解析，就通过这里拿到全局只读 ACPI 表。
    init();
    ACPI_TABLES.get().and_then(|tables| tables.as_ref())
}

fn detect_acpi_info() -> AcpiDeviceInfo {
    // 静态设备信息优先尝试从 MADT/HPET 推导；缺失时退回 qemu 静态配置。
    static_info::detect_static_acpi_info(ACPI_TABLES.get().and_then(|tables| tables.as_ref()))
}

fn detect_acpi_tables() -> Option<AcpiTables<AcpiHost>> {
    let host = AcpiHost;

    // 先在 BIOS 提供的 RSDP 里找到 ACPI 入口，再据此构造完整表视图。
    let rsdp = match unsafe { Rsdp::search_for_on_bios(host) } {
        Ok(rsdp) => rsdp,
        Err(e) => {
            log::warn!("ACPI RSDP not found ({:?})", e);
            return None;
        }
    };

    // 成功时把完整 ACPI 表缓存为全局只读对象，供 bus 阶段统一使用。
    match unsafe { AcpiTables::from_rsdp(host, rsdp.physical_start) } {
        Ok(tables) => Some(tables),
        Err(e) => {
            log::warn!("Failed to parse ACPI tables ({:?})", e);
            None
        }
    }
}