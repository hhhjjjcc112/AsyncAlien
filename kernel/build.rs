use std::{env, fs, path::{Path, PathBuf}};

const PLATFORM_QEMU_RISCV: &str = "plat_qemu_riscv";
const PLATFORM_QEMU_X86_64: &str = "plat_qemu_x86_64";
const PLATFORM_VF2: &str = "plat_vf2";
const VALID_PLATFORMS: [&str; 3] = [PLATFORM_QEMU_RISCV, PLATFORM_QEMU_X86_64, PLATFORM_VF2];

#[derive(Clone, Copy)]
enum TargetArch {
    Riscv64,
    X86_64,
}

impl TargetArch {
    fn parse(raw: &str) -> Self {
        match raw {
            "riscv64" => Self::Riscv64,
            "x86_64" => Self::X86_64,
            other => panic!(
                "Unsupported target architecture '{}'. Expected x86_64 or riscv64",
                other
            ),
        }
    }

    fn output_arch(self) -> &'static str {
        match self {
            Self::Riscv64 => "riscv",
            Self::X86_64 => "i386:x86-64",
        }
    }

    fn default_platform(self) -> &'static str {
        match self {
            Self::Riscv64 => PLATFORM_QEMU_RISCV,
            Self::X86_64 => PLATFORM_QEMU_X86_64,
        }
    }

    fn base_address(self, platform: &str) -> usize {
        match self {
            Self::X86_64 => 0x20_0000,
            Self::Riscv64 if platform == PLATFORM_VF2 => 0x4020_0000,
            Self::Riscv64 => 0x8020_0000,
        }
    }
}

fn resolve_platform(target_arch: TargetArch) -> String {
    let platform = env::var("PLATFORM")
        .ok()
        .unwrap_or_else(|| target_arch.default_platform().to_string());

    if !VALID_PLATFORMS.contains(&platform.as_str()) {
        panic!(
            "Invalid PLATFORM='{}'. Valid values are: {:?}",
            platform, VALID_PLATFORMS
        );
    }

    let valid_combo = match target_arch {
        TargetArch::X86_64 => platform == PLATFORM_QEMU_X86_64,
        TargetArch::Riscv64 => matches!(platform.as_str(), PLATFORM_QEMU_RISCV | PLATFORM_VF2),
    };

    if !valid_combo {
        panic!(
            "Invalid ARCH/PLATFORM combination: target_arch='{}', PLATFORM='{}'",
            env::var("CARGO_CFG_TARGET_ARCH").ok().unwrap_or_default(),
            platform
        );
    }

    platform
}

fn render_link_script(template: &str, target_arch: TargetArch, platform: &str) -> String {
    let base_address = format!("{:#x}", target_arch.base_address(platform));
    template
        .replace("{{arch}}", target_arch.output_arch())
        .replace("{{base_address}}", &base_address)
        .replace("{{max_cpu_num}}", "256")
}

fn write_link_script(out_dir: &str, content: &str) -> PathBuf {
    let link_script = Path::new(out_dir).join("link.lds");
    fs::write(&link_script, content).expect("failed to write generated linker script");
    link_script
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../tools/link.ld");
    println!("cargo:rerun-if-env-changed=PLATFORM");
    println!("cargo:rerun-if-env-changed=VF2_SD");

    println!("cargo::rustc-check-cfg=cfg(plat_qemu_riscv)");
    println!("cargo::rustc-check-cfg=cfg(plat_qemu_x86_64)");
    println!("cargo::rustc-check-cfg=cfg(plat_vf2)");
    println!("cargo::rustc-check-cfg=cfg(plat_vf2_sd)");

    let target_arch_raw = env::var("CARGO_CFG_TARGET_ARCH").ok().unwrap_or_default();
    let target_arch = TargetArch::parse(&target_arch_raw);
    let platform = resolve_platform(target_arch);

    // 统一按模板替换，避免占位符残留到最终链接脚本。
    let template = fs::read_to_string("../tools/link.ld").expect("failed to read tools/link.ld");
    let script_content = render_link_script(&template, target_arch, &platform);
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR is not set");
    let link_script = write_link_script(&out_dir, &script_content);

    if env::var("VF2_SD").ok().unwrap_or_else(|| "n".to_string()) == "y" {
        println!("cargo:rustc-cfg=plat_vf2_sd");
    }

    println!("cargo:rustc-link-arg=-T{}", link_script.display());
    println!("cargo::rustc-cfg={}", platform);
}
