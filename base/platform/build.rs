/// Valid platforms
const VALID_PLATFORMS: [&str; 3] = ["plat_qemu_riscv", "plat_qemu_x86_64", "plat_vf2"];

fn main() {
    let platform = option_env!("PLATFORM").unwrap_or("plat_qemu_riscv");
    
    // Validate platform
    if !VALID_PLATFORMS.contains(&platform) {
        panic!("Invalid PLATFORM='{}'. Valid values are: {:?}", platform, VALID_PLATFORMS);
    }
    
    println!("cargo::rustc-cfg={}", platform);
}
