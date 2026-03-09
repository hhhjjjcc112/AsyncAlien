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
FSMOUNT := ../../diskfs

all:build
	@echo "Moving apps to ../diskfs/bin"
	@$(foreach dir, $(BUILD_CRATES), (sudo cp $(target)$(dir) $(FSMOUNT)/$(dir););)


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