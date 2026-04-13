## 1. vDSO

vDSO（Virtual Dynamic Shared Object）是由内核映射到用户态的一段共享 ELF 代码，
用户程序可以直接调用其中的函数，避免频繁陷入内核 syscall。

常见做法是同时映射两块区域：
- `vVAR`：共享数据区（通常 UR）
- `vDSO`：共享代码区（通常 URX）

用户态通过 aux 向量的 `AT_SYSINFO_EHDR` 找到 vDSO ELF 基址。

## 2. 模板里两个 crate 的作用

上游模板仓库 [rosy233333/vdso_crate_template](https://github.com/rosy233333/vdso_crate_template) 里核心是两个 crate：

1. `vdso_helper`
- 提供 `vvar_data!`：定义共享数据结构（vVAR）
- 提供 `get_vvar_data!`：在 vDSO 代码内访问共享数据

2. `build_vdso`
- 在宿主工程的 `build.rs` 里调用
- 自动完成：
  - 生成 wrapper 工程
  - 构建 vDSO `.so`
  - 生成 API crate（给外部工程调用）

## 3. 最小使用步骤（推荐先跑通）

### 步骤 A：写一个 no_std 的 vDSO 功能 crate

要求：
- `#![no_std]`
- 对外函数放在 `api.rs`
- 导出函数写成：

```rust
#[unsafe(no_mangle)]
pub extern "C" fn my_vdso_fn(...) -> ... { ... }
```

如果需要共享数据：
- 在 crate 根模块用 `vvar_data!` 定义结构
- 在函数里用 `get_vvar_data!` 读取

### 步骤 B：在外部工程 build.rs 调用 build_vdso

典型形式：

```rust
use build_vdso::*;

fn main() {
    let mut config = BuildConfig::new("../my_vdso", "my_vdso");
    config.arch = "x86_64".to_string();
    config.out_dir = ".../out".to_string();
    config.so_name = "libmyvdso".to_string();
    config.api_lib_name = "libmyvdso".to_string();
    build_vdso(&config);
}
```

构建后会得到：
- `libmyvdso.so`（需要被映射到用户地址空间）
- `libmyvdso` API crate（用于解析并调用导出函数）

### 步骤 C：内核/任务域映射 vVAR + vDSO

创建用户进程地址空间时，按页对齐映射：
- 先映射 `vVAR`
- 再映射 `vDSO`

并在 auxv 中设置：
- `AT_SYSINFO_EHDR = vDSO ELF 基址`

### 步骤 D：用户态优先走 vDSO，失败回退 syscall

用户态读取 auxv 的 `AT_SYSINFO_EHDR` 后：
- 校验 ELF
- 查找目标符号（如 `__vdso_clock_gettime`）
- 成功则调用
- 失败则回退原 syscall

## 4. AsyncAlien 中建议的最小落地方式

先做最小可运行（MVP）：
- 仅实现 `clock_gettime` vDSO 快路径
- `gettimeofday/time/getcpu` 继续 syscall

建议接入点：
- 任务域地址空间构建处：新增 vVAR/vDSO 映射
- `AuxVec` 生成与入栈处：注入 `AT_SYSINFO_EHDR`
- `user/userlib` 时间接口：先尝试 vDSO，失败回退 syscall

## 5. 常见坑

1. `AT_SYSINFO_EHDR` 没传
- 用户态会直接找不到 vDSO，必须回退 syscall。

2. 映射权限不对
- vDSO 代码区应至少可执行（RX），vVAR 数据区应可读写（RW）。

3. 地址冲突
- 映射地址要避开现有用户栈、mmap 区、trampoline/trap 等关键区域。

4. 只实现了快路径，忘了回退
- 任何异常都必须 fallback 到 syscall，避免功能回归。

## 4. AsyncAlien 当前落地结构

- 源 crate：`vdso/vdso_impl/`
- 构建器：`vdso/vdso_builder/`
- 独立构建命令：`make vdso`（默认跟随 `ARCH`）
- 可调参数：`VDSO_TOOLCHAIN`、`VDSO_VERBOSE`；直接调用 builder 时也可以用 `VDSO_ARCH` 覆盖。
- x86_64 和 riscv64 现在都直接走本地 `vdso/build_vdso`，先产出 `staticlib`，再由对应架构的 musl `ld` 链接成 `.so`。
- 最终链接会自动生成版本脚本，并加上 `--gc-sections`，把 `PLT/GOT/INIT/FINI/RELA` 等多余段收掉。
- 产物输出：
  - `build/vdso/libvdso.so`
  - `build/vdso/vdso_api`
  - `build/vdso/vdso_wrapper`
  - `build/vdso/vdso_linker.lds`
- 当前状态：只导出 `__vdso_gettimeofday` 占位实现，后续在任务域和 userlib 完成映射与优先调用后，再扩展更多接口。

## 5. 参考资料

- vdso(7): https://man7.org/linux/man-pages/man7/vdso.7.html
- getauxval(3): https://man7.org/linux/man-pages/man3/getauxval.3.html
- AsyncModules 模板：[rosy233333/vdso_crate_template](https://github.com/rosy233333/vdso_crate_template)
