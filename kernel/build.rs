use std::{env, fs, fs::File, io::Write, path::Path};

/// Valid platforms
const VALID_PLATFORMS: [&str; 3] = ["plat_qemu_riscv", "plat_qemu_x86_64", "plat_vf2"];

fn main() {
    let outdir = env::var("OUT_DIR").unwrap();
    let link_script = Path::new(&outdir).join("link.lds");
    let mut script = File::create(&link_script).unwrap();
    let platform = option_env!("PLATFORM").unwrap_or("plat_qemu_riscv");
    
    // Validate platform
    if !VALID_PLATFORMS.contains(&platform) {
        panic!("Invalid PLATFORM='{}'. Valid values are: {:?}", platform, VALID_PLATFORMS);
    }
    
    // Choose linker script based on platform
    let ld_path = if platform == "plat_qemu_x86_64" {
        Path::new("../tools/link_x86_64.ld")
    } else {
        Path::new("../tools/link.ld")
    };
    let ld = fs::read_to_string(ld_path).unwrap();
    
    if platform == "plat_vf2" {
        let base_addr = 0x40200000;
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

    let plat_vf2_sd = option_env!("VF2_SD").unwrap_or("n");
    if plat_vf2_sd == "y" {
        println!("cargo:rustc-cfg=plat_vf2_sd");
    }

    println!("cargo:rustc-link-arg=-T{}", &link_script.display());
    println!("cargo::rustc-cfg={}", platform);
}
