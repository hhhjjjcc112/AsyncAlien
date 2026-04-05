ARCH ?= riscv64
TARGET_riscv64 := riscv64gc-unknown-linux-musl
TARGET_x86_64 := x86_64-unknown-linux-musl
TARGET := $(TARGET_$(ARCH))
LINKER_riscv64 := riscv64-linux-musl-gcc
LINKER_x86_64 := x86_64-linux-musl-gcc
LINKER := $(LINKER_$(ARCH))
LINKER_ENV_riscv64 := CARGO_TARGET_RISCV64GC_UNKNOWN_LINUX_MUSL_LINKER
LINKER_ENV_x86_64 := CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER
LINKER_ENV := $(LINKER_ENV_$(ARCH))
compile := $(LINKER_ENV)=$(LINKER) cargo build --release --target $(TARGET) -Z build-std=core,std,panic_abort
mode := release
target := ../../target/$(TARGET)/$(mode)/
IMG ?= ../../build/sdcard.img
MTOOLS_MCOPY ?= mcopy



all: build install

install:
	@command -v $(MTOOLS_MCOPY) >/dev/null 2>&1 || { echo "[ERR] 未找到 $(MTOOLS_MCOPY)，请安装 mtools"; exit 1; }
	@[ -f "$(IMG)" ] || { echo "[ERR] 镜像 $(IMG) 不存在，请先执行顶层 make fat/sdcard"; exit 1; }
	@echo "Moving apps to $(IMG)"
	@$(foreach dir, $(BUILD_CRATES), ($(MTOOLS_MCOPY) -D o -i $(IMG) $(target)$(dir) ::/$(dir););)


build:
	@echo "Building apps"
	$(foreach dir, $(BUILD_CRATES), ($(compile) -p $(dir));)


BUILD_CRATES := \
	hello \
	async_test \
	ftest \
	fs_update \
	pingpong
