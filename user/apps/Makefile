GUI ?=n
ARCH ?= riscv64
TARGET_riscv64 := riscv64gc-unknown-none-elf
TARGET_x86_64 := x86_64-unknown-none
TARGET := $(TARGET_$(ARCH))
APPS := $(shell ls -d */)
compile := cargo build --release --target $(TARGET)
mode := release
target := ../../target/$(TARGET)/$(mode)/
APPS_NAME := $(shell ls -d */ | cut -d '/' -f 1)
IMG ?= ../../build/sdcard.img
MTOOLS_MCOPY ?= mcopy
MTOOLS_MMD ?= mmd

all: build install

install:
	@command -v $(MTOOLS_MCOPY) >/dev/null 2>&1 || { echo "[ERR] 未找到 $(MTOOLS_MCOPY)，请安装 mtools"; exit 1; }
	@[ -f "$(IMG)" ] || { echo "[ERR] 镜像 $(IMG) 不存在，请先执行顶层 make fat/sdcard"; exit 1; }
	@$(MTOOLS_MMD) -i $(IMG) ::/tests >/dev/null 2>&1 || true
	@echo "Moving apps to $(IMG)"
	@for dir in $(BUILD_CRATES); do \
		if [ "$$dir" = "init" ]; then \
			$(MTOOLS_MCOPY) -D o -i $(IMG) $(target)$$dir ::/tests/init; \
		elif [ "$$dir" = "tests" ]; then \
			$(MTOOLS_MCOPY) -D o -i $(IMG) $(target)$$dir ::/tests/tests; \
		else \
			$(MTOOLS_MCOPY) -D o -i $(IMG) $(target)$$dir ::/$$dir; \
		fi; \
	done


build:
	@echo "Building apps"
	@echo $(BUILD_CRATES)
	@$(foreach dir, $(BUILD_CRATES), (cd $(dir) && $(compile) && cd ..);)


BUILD_CRATES := \
	init \
	dtest \
	dgpu \
	dtask \
	dlog \
	dinput \
	dnet \
	dvfs \
	ping \
	client \
	out \
	nice \
#	run_test \
	tests \
	sleep \
	socket_test \
	final_test \
	ls \
	cat \
	mkdir \
	print \

ifeq ($(GUI),y)
	BUILD_CRATES += slint \
					memory-game
endif