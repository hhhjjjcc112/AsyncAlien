use build_vdso::{build_vdso as build_vdso_tool, BuildConfig};
use std::{env, path::{Path, PathBuf}};

#[derive(Clone, Copy)]
enum TargetArch {
    Riscv64,
    X86_64,
    ARM64,
}

impl TargetArch {
    fn parse(raw: &str) -> Self {
        match raw {
            "riscv64" => Self::Riscv64,
            "x86_64" => Self::X86_64,
            "aarch64" | "arm64" => Self::ARM64,
            other => panic!(
                "不支持的 vDSO 架构 '{}', 只接受 x86_64 或 riscv64",
                other
            ),
        }
    }

    fn build_arch(self) -> &'static str {
        match self {
            Self::X86_64 => "x86_64",
            Self::Riscv64 => "riscv64",
            Self::ARM64 => "aarch64",
        }
    }
}

fn main() {
    let repo_root = repo_root();
    let arch = env::var("VDSO_ARCH")
        .or_else(|_| env::var("ARCH"))
        .unwrap_or_else(|_| "x86_64".to_string());
    let target_arch = TargetArch::parse(&arch);
    build_vdso_for_arch(target_arch, &repo_root);
}

fn repo_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR 未设置"));
    manifest_dir
        .parent()
        .and_then(|path| path.parent())
        .expect("vdso_builder 不在工具目录下")
        .to_path_buf()
}

fn build_vdso_for_arch(target_arch: TargetArch, repo_root: &Path) {
    let arch = target_arch.build_arch();
    let vdso_src_dir = env::var("VDSO_SRC_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| repo_root.join("vdso/vdso_impl"));
    let vdso_out_dir = env::var("VDSO_OUT_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| repo_root.join("build/vdso"));

    let mut config = BuildConfig::new(
        vdso_src_dir
            .to_str()
            .expect("vdso 源目录不是有效 UTF-8"),
        "vdso_impl",
    );
    config.arch = arch.to_string();
    config.out_dir = vdso_out_dir
        .to_str()
        .expect("vdso 输出目录不是有效 UTF-8")
        .to_string();
    config.so_name = env::var("VDSO_SO_NAME").unwrap_or_else(|_| "libvdso".to_string());
    config.api_lib_name = env::var("VDSO_API_LIB_NAME").unwrap_or_else(|_| "vdso_api".to_string());
    config.toolchain = env::var("VDSO_TOOLCHAIN").unwrap_or_else(|_| "nightly-2026-01-23".to_string());
    config.verbose = env::var("VDSO_VERBOSE")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(0);

    // 两个架构现在都走本地 staticlib+ld 链路。
    println!("[vdso_builder] 构建 arch={arch} src={} out={}", config.src_dir, config.out_dir);
    build_vdso_tool(&config);
    println!("[vdso_builder] 构建完成");
}