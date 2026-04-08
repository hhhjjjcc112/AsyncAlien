GUI ?=n
ARCH ?= riscv64
TARGET_riscv64 := riscv64gc-unknown-none-elf
TARGET_x86_64 := x86_64-unknown-none
TARGET := $(TARGET_$(ARCH))
compile := cargo build --release --target $(TARGET)
mode := release
target := ../../target/$(TARGET)/$(mode)/
IMG ?= ../../build/sdcard.img
MTOOLS_MCOPY ?= mcopy

all: build install

install:
	@command -v $(MTOOLS_MCOPY) >/dev/null 2>&1 || { echo "[ERR] 未找到 $(MTOOLS_MCOPY)，请安装 mtools"; exit 1; }
	@[ -f "$(IMG)" ] || { echo "[ERR] 镜像 $(IMG) 不存在，请先执行顶层 make fat/sdcard"; exit 1; }
	@echo "Moving tests to $(IMG)"
	@$(foreach dir, $(BUILD_CRATES), ($(MTOOLS_MCOPY) -D o -i $(IMG) $(target)$(notdir $(dir)) ::/$(notdir $(dir)););)

build:
	@echo "Building tests"
	@echo $(BUILD_CRATES)
	@$(foreach dir, $(BUILD_CRATES), (cd $(dir) && $(compile) && cd ../..);)

BUILD_CRATES := \
	new/sys_time \
	new/sys_fs \
	new/sys_link \
	new/sys_proc \
	new/syscall_all \

ifeq ($(GUI),y)
	BUILD_CRATES +=
endif
