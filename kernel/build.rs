use std::{env, fs, fs::File, io::Write, path::Path};

/// Valid platforms
const VALID_PLATFORMS: [&str; 3] = ["plat_qemu_riscv", "plat_qemu_x86_64", "plat_vf2"];

fn main() {
    println!("cargo::rustc-check-cfg=cfg(plat_qemu_riscv)");
    println!("cargo::rustc-check-cfg=cfg(plat_qemu_x86_64)");
    println!("cargo::rustc-check-cfg=cfg(plat_vf2)");
    println!("cargo::rustc-check-cfg=cfg(plat_vf2_sd)");

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").ok().unwrap_or_default();

    let outdir = env::var("OUT_DIR").unwrap();
    let link_script = Path::new(&outdir).join("link.lds");
    let mut script = File::create(&link_script).unwrap();
    let platform = env::var("PLATFORM").ok().unwrap_or_else(|| match target_arch.as_str() {
        "x86_64" => "plat_qemu_x86_64".to_string(),
        "riscv64" => "plat_qemu_riscv".to_string(),
        _ => "plat_qemu_riscv".to_string(),
    });
    
    // Validate platform
    if !VALID_PLATFORMS.contains(&platform.as_str()) {
        panic!("Invalid PLATFORM='{}'. Valid values are: {:?}", platform, VALID_PLATFORMS);
    }

    match target_arch.as_str() {
        "x86_64" | "riscv64" => {}
        other => panic!("Unsupported target architecture '{}'. Expected x86_64 or riscv64", other),
    }

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
    
    let ld_path = Path::new("../tools/link.ld");
    let ld = fs::read_to_string(ld_path).unwrap();
    // 根据架构替换链接脚本中的占位符
    let ld = if target_arch == "x86_64" {
        ld.replace("{{arch}}", "i386:x86-64")
    } else {
        ld.replace("{{arch}}", "riscv")
    };
    
    let base_addr_override = if target_arch == "x86_64" {
        Some(0x20_0000usize)
    } else if platform == "plat_vf2" {
        Some(0x4020_0000usize)
    } else {
        None
    };

    if let Some(base_addr) = base_addr_override {
        let base_addr = format!("BASE_ADDRESS = {};", base_addr);
        let mut new_config = String::new();
        for line in ld.lines() {
            if line.starts_with("BASE_ADDRESS = ") {
                new_config.push_str(base_addr.as_str());
            } else {
                new_config.push_str(line);
                new_config.push('\n');
            }
        }
        script.write_all(new_config.as_bytes()).unwrap();
    } else {
        script.write_all(ld.as_bytes()).unwrap();
    }

    let plat_vf2_sd = env::var("VF2_SD").ok().unwrap_or_else(|| "n".to_string());
    if plat_vf2_sd == "y" {
        println!("cargo:rustc-cfg=plat_vf2_sd");
    }

    println!("cargo:rustc-link-arg=-T{}", &link_script.display());
    println!("cargo::rustc-cfg={}", platform.as_str());
}
