/// Valid platforms
const VALID_PLATFORMS: [&str; 3] = ["plat_qemu_riscv", "plat_qemu_x86_64", "plat_vf2"];

fn main() {
    println!("cargo::rustc-check-cfg=cfg(plat_qemu_riscv)");
    println!("cargo::rustc-check-cfg=cfg(plat_qemu_x86_64)");
    println!("cargo::rustc-check-cfg=cfg(plat_vf2)");

    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").ok().unwrap_or_default();

    let platform = std::env::var("PLATFORM").ok().unwrap_or_else(|| {
        match target_arch.as_str() {
            "x86_64" => "plat_qemu_x86_64".to_string(),
            "riscv64" => "plat_qemu_riscv".to_string(),
            _ => "plat_qemu_riscv".to_string(),
        }
    });
    
    // Validate platform
    if !VALID_PLATFORMS.contains(&platform.as_str()) {
        panic!("Invalid PLATFORM='{}'. Valid values are: {:?}", platform, VALID_PLATFORMS);
    }

    // Validate target architecture.
    match target_arch.as_str() {
        "x86_64" | "riscv64" => {}
        other => panic!("Unsupported target architecture '{}'. Expected x86_64 or riscv64", other),
    }

    // Validate arch-platform combination.
    let is_valid_combo = match target_arch.as_str() {
        "x86_64" => platform == "plat_qemu_x86_64",
        "riscv64" => matches!(platform.as_str(), "plat_qemu_riscv" | "plat_vf2"),
        _ => false,
    };
    if !is_valid_combo {
        panic!(
            "Invalid ARCH/PLATFORM combination: target_arch='{}', PLATFORM='{}'",
            target_arch, platform
        );
    }
    
    println!("cargo::rustc-cfg={}", platform);
}
