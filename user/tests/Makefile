GUI ?=n
ARCH ?= riscv64
TARGET_riscv64 := riscv64gc-unknown-none-elf
TARGET_x86_64 := x86_64-unknown-none
TARGET := $(TARGET_$(ARCH))
USERLIB_EXTRA_RUSTFLAGS ?=
RUSTFLAGS_PREFIX :=
ifneq ($(strip $(USERLIB_EXTRA_RUSTFLAGS)),)
RUSTFLAGS_PREFIX := RUSTFLAGS="$$RUSTFLAGS $(USERLIB_EXTRA_RUSTFLAGS)"
endif
compile := $(RUSTFLAGS_PREFIX) cargo build --release --target $(TARGET)
mode := release
target := ../../target/$(TARGET)/$(mode)/
IMG ?= ../../build/sdcard.img
MTOOLS_MCOPY ?= mcopy
MTOOLS_MDIR ?= mdir
MTOOLS_MMD ?= mmd

all: build install

install:
	@command -v $(MTOOLS_MCOPY) >/dev/null 2>&1 || { echo "[ERR] 未找到 $(MTOOLS_MCOPY)，请安装 mtools"; exit 1; }
	@[ -f "$(IMG)" ] || { echo "[ERR] 镜像 $(IMG) 不存在，请先执行顶层 make fat/sdcard"; exit 1; }
	@$(MTOOLS_MDIR) -i $(IMG) ::/new >/dev/null 2>&1 || $(MTOOLS_MMD) -i $(IMG) ::/new >/dev/null 2>&1 || true
	@echo "Moving tests to $(IMG)"
	@$(foreach dir, $(BUILD_CRATES), ($(MTOOLS_MCOPY) -D o -i $(IMG) $(target)$(notdir $(dir)) ::/new/$(notdir $(dir)););)

build:
	@echo "Building tests"
	@echo $(BUILD_CRATES)
	@$(foreach dir, $(BUILD_CRATES), (cd $(dir) && $(compile) && cd ../..);)

BUILD_CRATES := \
	new/sys_time \
	new/sys_vdso_time_compare \
	new/sys_fs \
	new/sys_link \
	new/sys_proc \
	new/sys_poll \
	new/sys_random \
	new/syscall_all \

ifeq ($(GUI),y)
	BUILD_CRATES +=
endif
