## Architecture and platform selection
## ARCH: riscv64 (default), x86_64, or legacy vf2
## PLATFORM: plat_qemu_riscv / plat_qemu_x86_64 / plat_vf2
ARCH ?= riscv64
PLATFORM ?=

# Validate ARCH value
VALID_ARCHS := riscv64 x86_64 vf2
ifeq ($(filter $(ARCH),$(VALID_ARCHS)),)
$(error Invalid ARCH=$(ARCH). Valid values are: $(VALID_ARCHS))
endif

# Validate PLATFORM value when explicitly provided
VALID_PLATFORMS := plat_qemu_riscv plat_qemu_x86_64 plat_vf2
ifneq ($(strip $(PLATFORM)),)
ifeq ($(filter $(PLATFORM),$(VALID_PLATFORMS)),)
$(error Invalid PLATFORM=$(PLATFORM). Valid values are: $(VALID_PLATFORMS))
endif
endif

# Normalize arch selection (keep legacy ARCH=vf2 compatible)
ifeq ($(ARCH),x86_64)
	ARCH_KIND := x86_64
	PLATFORM_DEFAULT := plat_qemu_x86_64
else ifeq ($(ARCH),vf2)
	ARCH_KIND := riscv64
	PLATFORM_DEFAULT := plat_vf2
else
	ARCH_KIND := riscv64
	PLATFORM_DEFAULT := plat_qemu_riscv
endif

# Fill default platform if not provided
ifeq ($(strip $(PLATFORM)),)
	PLATFORM := $(PLATFORM_DEFAULT)
endif

# Validate ARCH + PLATFORM combinations
ifeq ($(ARCH_KIND),x86_64)
ifneq ($(PLATFORM),plat_qemu_x86_64)
$(error ARCH=x86_64 only supports PLATFORM=plat_qemu_x86_64)
endif
else
VALID_RISCV_PLATFORMS := plat_qemu_riscv plat_vf2
ifeq ($(filter $(PLATFORM),$(VALID_RISCV_PLATFORMS)),)
$(error ARCH=$(ARCH) only supports PLATFORM in $(VALID_RISCV_PLATFORMS))
endif
endif

ifeq ($(ARCH_KIND),x86_64)
	TARGET := x86_64-unknown-none
	TARGET_CONFIG := ./tools/x86_64.json
	TARGET2 := x86_64
	QEMU := qemu-system-x86_64
else
	TARGET := riscv64gc-unknown-none-elf
	TARGET_CONFIG := ./tools/riscv64.json
	TARGET2 := riscv64
	QEMU := qemu-system-riscv64
endif

PROFILE := release
KERNEL := target/$(TARGET2)/$(PROFILE)/kernel
NET ?= y
NET_HOSTFWD ?= y
NET_FWD_PORT_TCP ?= 55555
NET_FWD_PORT_UDP ?= 5555
SMP ?= 4
MEMORY_SIZE := 2048M
X86_MACHINE ?= q35
X86_ACCEL ?= tcg
X86_CPU_TCG ?= max
X86_CPU_KVM ?= host,+x2apic
ifeq ($(X86_ACCEL),kvm)
X86_CPU ?= $(X86_CPU_KVM)
else
X86_CPU ?= $(X86_CPU_TCG)
endif
X86_UINTR_CPU ?= max,+uintr
VIRTIO_FORCE_LEGACY ?= y
LOG ?=
GUI ?=n
FS ?=fat
MTOOLS_MCOPY ?= mcopy
MTOOLS_MMD ?= mmd
MTOOLS_MDIR ?= mdir
IMG := build/sdcard.img
FSMOUNT := ./diskfs
BASE_FEATURES ?= default
EXTRA_FEATURES ?=
APIC_TIMER_TEST ?= n
UNWIND_TEST ?= n
MEMORY_TEST ?= n
TRAP_TEST ?= n
DOMAIN_TEST ?= n
DOMAIN_SYSCALL_TEST ?= n
DOMAIN_TASK_TEST ?= n
DOMAIN_APIC_TEST ?= n
DOMAIN_UART_TEST ?= n
DOMAIN_BLOCK_TEST ?= n
DOMAIN_PROFILE ?=
name ?=
VF2 ?= n
TFTPBOOT := /home/godones/projects/tftpboot/
VF2_SD ?= n
VDSO_TOOLCHAIN ?= nightly-2026-01-23
VDSO_VERBOSE ?= 0
BUILD_CFG ?=  -Z build-std=core,alloc -Z build-std-features=compiler-builtins-mem
BENCH ?= n
RUN_TIMEOUT ?= 60s
USE_INT80_SYSCALL ?= n
USER_INITRD_DIR := user/initrd
USER_INITRAMFS_DIR := $(USER_INITRD_DIR)/initramfs-$(ARCH_KIND)
USER_INITRD_STAMP := build/.user_initrd_$(ARCH_KIND).stamp
comma:= ,
empty:=
space:= $(empty) $(empty)
USERLIB_EXTRA_RUSTFLAGS :=

ifeq ($(ARCH_KIND),x86_64)
ifeq ($(USE_INT80_SYSCALL),y)
USERLIB_EXTRA_RUSTFLAGS := --cfg int80_syscall --check-cfg=cfg(int80_syscall)
endif
endif

QEMU_ARGS :=

ifeq ($(ARCH_KIND),x86_64)
    # x86_64 QEMU args
	QEMU_ARGS += -machine $(X86_MACHINE),accel=$(X86_ACCEL)
    VIRTIO_PCI_OPTS :=
    ifeq ($(VIRTIO_FORCE_LEGACY),y)
        VIRTIO_PCI_OPTS := ,disable-modern=on,disable-legacy=off,x-disable-pcie=on
    endif
    ifeq ($(GUI),y)
        QEMU_ARGS += -device virtio-gpu-pci$(VIRTIO_PCI_OPTS) \
                     -device virtio-keyboard-pci$(VIRTIO_PCI_OPTS) \
                     -device virtio-mouse-pci$(VIRTIO_PCI_OPTS)
    else
        QEMU_ARGS += -nographic
    endif
    ifeq ($(NET),y)
        ifeq ($(NET_HOSTFWD),y)
            QEMU_ARGS += -device virtio-net-pci,netdev=net0$(VIRTIO_PCI_OPTS) \
                         -netdev user,id=net0,hostfwd=tcp::$(NET_FWD_PORT_TCP)-:$(NET_FWD_PORT_TCP),hostfwd=udp::$(NET_FWD_PORT_UDP)-:$(NET_FWD_PORT_UDP)
        else
            QEMU_ARGS += -device virtio-net-pci,netdev=net0$(VIRTIO_PCI_OPTS) \
                         -netdev user,id=net0
        endif
    endif
    QEMU_ARGS += -drive file=$(IMG),if=none,format=raw,id=x0 \
                 -device virtio-blk-pci,drive=x0$(VIRTIO_PCI_OPTS)
else
    # riscv64 QEMU args
    ifeq ($(GUI),y)
        QEMU_ARGS += -device virtio-gpu-device \
                     -device virtio-tablet-device \
                     -device virtio-keyboard-device
    else
        QEMU_ARGS += -nographic
    endif
    ifeq ($(NET),y)
        ifeq ($(NET_HOSTFWD),y)
            QEMU_ARGS += -device virtio-net-device,netdev=net0 \
                         -netdev user,id=net0,hostfwd=tcp::$(NET_FWD_PORT_TCP)-:$(NET_FWD_PORT_TCP),hostfwd=udp::$(NET_FWD_PORT_UDP)-:$(NET_FWD_PORT_UDP)
        else
            QEMU_ARGS += -device virtio-net-device,netdev=net0 \
                         -netdev user,id=net0
        endif
    endif
    QEMU_ARGS += -drive file=$(IMG),if=none,format=raw,id=x0 \
                 -device virtio-blk-device,drive=x0
endif

QEMU_ARGS += -initrd ./build/initramfs.cpio.gz
QEMU_ARGS += -append "rdinit=/init"

ifeq ($(BENCH),y)
EXTRA_FEATURES += bench
endif

FEATURE_TEST_FLAGS :=
ifeq ($(APIC_TIMER_TEST),y)
FEATURE_TEST_FLAGS += apic_timer_test
endif

ifeq ($(UNWIND_TEST),y)
FEATURE_TEST_FLAGS += unwind_test
endif

ifeq ($(MEMORY_TEST),y)
FEATURE_TEST_FLAGS += memory_test
endif

ifeq ($(TRAP_TEST),y)
FEATURE_TEST_FLAGS += trap_test
endif

ifeq ($(DOMAIN_TEST),y)
FEATURE_TEST_FLAGS += domain_test
endif

ifeq ($(DOMAIN_SYSCALL_TEST),y)
FEATURE_TEST_FLAGS += domain_syscall_test
endif

ifeq ($(DOMAIN_TASK_TEST),y)
FEATURE_TEST_FLAGS += domain_task_test
endif

ifeq ($(DOMAIN_APIC_TEST),y)
FEATURE_TEST_FLAGS += domain_apic_test
endif

ifeq ($(DOMAIN_UART_TEST),y)
FEATURE_TEST_FLAGS += domain_uart_test
endif

ifeq ($(DOMAIN_BLOCK_TEST),y)
FEATURE_TEST_FLAGS += domain_block_test
endif

FEATURES := $(strip $(BASE_FEATURES) $(EXTRA_FEATURES) $(FEATURE_TEST_FLAGS))
FEATURES := $(subst $(space),$(comma),$(FEATURES))

export ARCH
export SMP
export PLATFORM
export VF2_SD

all:run

help:
	@echo "AsyncAlien Build System"
	@echo ""
	@echo "Architecture Selection:"
	@echo "  make ARCH=riscv64 ...                           Build for RISC-V 64-bit (default)"
	@echo "  make ARCH=x86_64 ...                            Build for x86-64"
	@echo "  make ARCH=vf2 ...                               Legacy alias for VF2"
	@echo ""
	@echo "Platform Selection:"
	@echo "  make ARCH=riscv64 PLATFORM=plat_qemu_riscv ... Build for RISC-V QEMU"
	@echo "  make ARCH=riscv64 PLATFORM=plat_vf2 ...        Build for VisionFive 2"
	@echo "  make ARCH=x86_64 PLATFORM=plat_qemu_x86_64 ... Build for x86-64 QEMU"
	@echo ""
	@echo "Main Targets:"
	@echo "  make run                Build and run in QEMU"
	@echo "  make record_run         Build, run, and tee output to run/run_$(ARCH).txt"
	@echo "  make ready              Build everything but don't run QEMU"
	@echo "  make build              Build kernel and vDSO"
	@echo "  make vdso               Build vDSO only (follows ARCH)"
	@echo "  make vf2                Build and deploy to VF2 via TFTP"
	@echo "  make domains            Build all domains"
	@echo "  make clean              Clean build artifacts"
	@echo ""
	@echo "Debug Targets:"
	@echo "  make gdb-server         Run QEMU with GDB server"
	@echo "  make gdb-client         Connect GDB client"
	@echo "  make kernel_asm         Disassemble kernel"
	@echo ""
	@echo "Options:"
	@echo "  SMP=n                   Number of CPUs (default: 2)"
	@echo "  NET=y/n                 Enable network (default: y)"
	@echo "  X86_MACHINE=...         x86 machine type (default: q35)"
	@echo "  X86_ACCEL=tcg/kvm       x86 accelerator (default: tcg)"
	@echo "  X86_CPU=...             x86 CPU model/features (default: max@tcg, host,+x2apic@kvm)"
	@echo "  GUI=y/n                 Enable GUI (default: n)"
	@echo "  LOG=level               Log level"
	@echo "  BASE_FEATURES=...       Base cargo features (default: default)"
	@echo "  EXTRA_FEATURES=...      Extra cargo features, space separated"
	@echo "  APIC_TIMER_TEST=y/n     Enable apic_timer_test feature"
	@echo "  UNWIND_TEST=y/n         Enable unwind_test feature"
	@echo "  MEMORY_TEST=y/n         Enable memory_test feature"
	@echo "  TRAP_TEST=y/n           Enable trap_test feature"
	@echo "  DOMAIN_TEST=y/n         Enable domain_test umbrella feature"
	@echo "  DOMAIN_SYSCALL_TEST=y/n Enable domain_syscall_test feature"
	@echo "  DOMAIN_TASK_TEST=y/n    Enable domain_task_test feature"
	@echo "  DOMAIN_APIC_TEST=y/n    Enable domain_apic_test feature"
	@echo "  DOMAIN_UART_TEST=y/n    Enable domain_uart_test feature"
	@echo "  DOMAIN_BLOCK_TEST=y/n   Enable domain_block_test feature"
	@echo "  USE_INT80_SYSCALL=y/n   Switch x86_64 userlib to int 0x80"
	@echo "  VF2_SD=y/n              Enable VF2 SD card support (default: n)"
	@echo "  VDSO_TOOLCHAIN=...      vDSO toolchain (default: nightly-2026-01-23)"
	@echo "  VDSO_VERBOSE=0/1/2      vDSO build verbosity"
	@echo ""
	@echo "Examples:"
	@echo "  make ARCH=riscv64 PLATFORM=plat_qemu_riscv run"
	@echo "  make ARCH=x86_64 PLATFORM=plat_qemu_x86_64 build SMP=4"
	@echo "  make ARCH=x86_64 run_uintr X86_UINTR_CPU=\"max,+x2apic,+uintr\""
	@echo "  make ARCH=riscv64 PLATFORM=plat_vf2 build VF2_SD=y"

# Build everything but don't run QEMU (same as run without the QEMU step)
ready: domains sdcard initrd build
	@echo "Build complete. Ready to run with 'make fake_run' or 'make run'"

build: vdso
	@echo "Building..."
	@echo "ARCH: $(ARCH)"
	@echo "PLATFORM: $(PLATFORM)"
	@echo "TARGET: $(TARGET)"
	@echo "FEATURES: $(FEATURES)"
	@echo "SMP: $(SMP)"
	@echo "VF2_SD: $(VF2_SD)"
	@#LOG=$(LOG) cargo build --release -p kernel --target $(TARGET) --features $(FEATURES)
	PLATFORM=$(PLATFORM) RUSTFLAGS='--cfg getrandom_backend="custom" --cfg $(PLATFORM)' LOG=$(LOG) cargo build --release -p kernel --target $(TARGET_CONFIG) $(BUILD_CFG) --features $(FEATURES)

vdso_clean_cache:
	@rm -rf ./build/vdso/target ./build/vdso/vdso_wrapper ./build/vdso/libvdso.so ./build/vdso/vdso_linker.lds ./build/vdso/vdso_version.map ./build/vdso/vdso_api

vdso_api_bootstrap:
	@if [ ! -f ./build/vdso/vdso_api/Cargo.toml ]; then \
		echo "Bootstrapping placeholder vdso_api crate for Cargo workspace resolution..."; \
		mkdir -p ./build/vdso/vdso_api/src; \
		printf '%s\n' '[package]' 'name = "vdso_api"' 'version = "0.0.0"' 'edition = "2021"' '' '[lib]' 'path = "src/lib.rs"' > ./build/vdso/vdso_api/Cargo.toml; \
		printf '%s\n' 'compile_error!("placeholder vdso_api crate: run `make vdso` to generate real API");' > ./build/vdso/vdso_api/src/lib.rs; \
	fi

vdso: vdso_clean_cache vdso_api_bootstrap
	@echo "Building vDSO..."
	VDSO_TOOLCHAIN=$(VDSO_TOOLCHAIN) VDSO_VERBOSE=$(VDSO_VERBOSE) cargo run --release -p vdso_builder

vf2:
	@$(MAKE) ARCH=riscv64 PLATFORM=plat_vf2 build
	rust-objcopy --strip-all target/riscv64/release/kernel -O binary ./testos.bin
	cp ./testos.bin  $(TFTPBOOT)
	rm ./testos.bin

ifeq ($(ARCH_KIND),x86_64)
# x86_64 run target
run: domains sdcard initrd build
	$(QEMU) \
            -m $(MEMORY_SIZE) \
            -smp $(SMP) \
			-cpu $(X86_CPU) \
            -kernel $(KERNEL) \
            $(QEMU_ARGS) \
            -serial mon:stdio

fake_run:
	$(QEMU) \
            -m $(MEMORY_SIZE) \
            -smp $(SMP) \
			-cpu $(X86_CPU) \
            -kernel $(KERNEL) \
            $(QEMU_ARGS) \
            -serial mon:stdio

record_run: domains sdcard initrd build
	@mkdir -p ./run
	@bash -lc 'set -o pipefail; timeout --foreground $(RUN_TIMEOUT) $(QEMU) \
            -m $(MEMORY_SIZE) \
            -smp $(SMP) \
			-cpu $(X86_CPU) \
            -kernel $(KERNEL) \
            $(QEMU_ARGS) \
            -serial mon:stdio > ./run/run_$(ARCH).txt 2>&1'

run_uintr: X86_CPU := $(X86_UINTR_CPU)
run_uintr: run

fake_run_uintr: X86_CPU := $(X86_UINTR_CPU)
fake_run_uintr: fake_run
else
# riscv64 run target
run: domains sdcard initrd build
	$(QEMU) \
            -M virt \
            -bios default \
            -kernel $(KERNEL) \
            $(QEMU_ARGS) \
            -smp $(SMP) -m $(MEMORY_SIZE) \
            -serial mon:stdio

fake_run:
	$(QEMU) \
			-M virt \
			-bios default \
			-kernel $(KERNEL) \
			$(QEMU_ARGS) \
			-smp $(SMP) -m $(MEMORY_SIZE) \
			-serial mon:stdio

record_run: domains sdcard initrd build
	@mkdir -p ./run
	@bash -lc 'set -o pipefail; timeout --foreground $(RUN_TIMEOUT) $(QEMU) \
			-M virt \
			-bios default \
			-kernel $(KERNEL) \
			$(QEMU_ARGS) \
			-smp $(SMP) -m $(MEMORY_SIZE) \
			-serial mon:stdio > ./run/run_$(ARCH).txt 2>&1'
endif

check_mtools:
	@command -v $(MTOOLS_MCOPY) >/dev/null 2>&1 || { echo "[ERR] 未找到 $(MTOOLS_MCOPY)，请安装 mtools"; exit 1; }
	@command -v $(MTOOLS_MMD) >/dev/null 2>&1 || { echo "[ERR] 未找到 $(MTOOLS_MMD)，请安装 mtools"; exit 1; }
	@command -v $(MTOOLS_MDIR) >/dev/null 2>&1 || { echo "[ERR] 未找到 $(MTOOLS_MDIR)，请安装 mtools"; exit 1; }

user: vdso check_mtools
	@echo "Building user apps"
	@make all -C ./user/apps ARCH=$(ARCH_KIND) IMG=$(abspath $(IMG)) MTOOLS_MCOPY=$(MTOOLS_MCOPY) USERLIB_EXTRA_RUSTFLAGS="$(USERLIB_EXTRA_RUSTFLAGS)"
	@make all -C ./user/tests ARCH=$(ARCH_KIND) IMG=$(abspath $(IMG)) MTOOLS_MCOPY=$(MTOOLS_MCOPY) USERLIB_EXTRA_RUSTFLAGS="$(USERLIB_EXTRA_RUSTFLAGS)"
	@make all -C ./user/musl ARCH=$(ARCH_KIND) IMG=$(abspath $(IMG)) MTOOLS_MCOPY=$(MTOOLS_MCOPY) USERLIB_EXTRA_RUSTFLAGS="$(USERLIB_EXTRA_RUSTFLAGS)"
	@echo "Building user apps done"


sdcard:$(FS) check_mtools user #domains
	@echo "[sdcard] 使用 mtools 写入 $(IMG)"
	@if [ -n "$$(find build/disk -mindepth 1 -maxdepth 1 -print -quit)" ]; then \
		$(MTOOLS_MCOPY) -D o -i $(IMG) build/disk/* ::; \
	else \
		echo "[WARN] build/disk 为空，跳过写入"; \
	fi
	@if [ -n "$$(find user/bin -mindepth 1 -maxdepth 1 -print -quit)" ]; then \
		$(MTOOLS_MCOPY) -D o -i $(IMG) user/bin/* ::; \
	else \
		echo "[sdcard] user/bin 为空，跳过写入"; \
	fi
	@$(MTOOLS_MMD) -i $(IMG) ::/domains >/dev/null 2>&1 || true
	@$(MTOOLS_MDIR) -i $(IMG) ::

fat:
	dd if=/dev/zero of=$(IMG) bs=1M count=72;
	@mkfs.fat -F 32 $(IMG)


mount:
	@echo "Mounting $(IMG) to $(FSMOUNT)"
	@-sudo umount $(FSMOUNT);
	@sudo rm -rf $(FSMOUNT)
	mkdir $(FSMOUNT)
	@sudo mount $(IMG) $(FSMOUNT)
	@sudo rm -rf $(FSMOUNT)/*


domains:
	@if [ ! -d "build" ]; then mkdir build; fi
	cd domains && ARCH=$(ARCH) PLATFORM=$(PLATFORM) DOMAIN_PROFILE=$(DOMAIN_PROFILE) cargo domain build-all -l "$(LOG)" -o $(abspath build)

domain:
	cd domains && ARCH=$(ARCH) PLATFORM=$(PLATFORM) DOMAIN_PROFILE=$(DOMAIN_PROFILE) cargo domain build -n $(name) -l "$(LOG)" -o $(abspath build)

initrd:
	@need_rebuild=0; \
	if [ ! -d "$(USER_INITRAMFS_DIR)" ] || [ ! -e "$(USER_INITRAMFS_DIR)/bin/busybox" ]; then \
		need_rebuild=1; \
	elif [ ! -f "$(USER_INITRD_STAMP)" ]; then \
		need_rebuild=1; \
	elif [ -n "$$(find $(USER_INITRD_DIR) -type f -newer "$(USER_INITRD_STAMP)" -print -quit)" ]; then \
		need_rebuild=1; \
	fi; \
	if [ $$need_rebuild -eq 1 ]; then \
		echo "[initrd] build user/initrd ARCH=$(ARCH_KIND) (auto-detected)"; \
		if make -C $(USER_INITRD_DIR) ARCH=$(ARCH_KIND); then \
			mkdir -p ./build; \
			touch "$(USER_INITRD_STAMP)"; \
		else \
			echo "[WARN] user/initrd 构建失败，回退使用现有 initramfs-$(ARCH_KIND)"; \
		fi; \
	else \
		echo "[initrd] skip user/initrd rebuild (up-to-date)"; \
	fi
	@mkdir -p ./initrd
	@cp ./build/init/g* ./initrd
	@if [ "$(ARCH_KIND)" = "x86_64" ]; then \
		rm -f ./initrd/gvirtio_input ./initrd/gvirtio_gpu; \
	else \
		rm -f ./initrd/gvirtio_input ./initrd/gvirtio_gpu; \
	fi
	@if [ -d ./user/initrd/initramfs-$(ARCH_KIND) ]; then \
		cp ./user/initrd/initramfs-$(ARCH_KIND)/* ./initrd -r; \
	else \
		echo "[WARN] ./user/initrd/initramfs-$(ARCH_KIND) 不存在，回退到旧目录"; \
		cp ./user/initrd/initramfs/* ./initrd -r; \
	fi
	@#-cp ./user/bin/* ./initrd/bin -r
	@#cd ./initrd && find . -print0 | cpio --null -ov --format=newc | gzip -9 > ../build/initramfs.cpio.gz && cd ..
	@cd ./initrd && find . | cpio -o -H newc | gzip -9 > ../build/initramfs.cpio.gz && cd ..
	@rm -rf ./initrd


ifeq ($(ARCH_KIND),x86_64)
gdb-server: domains build sdcard
	@$(QEMU) \
            -m $(MEMORY_SIZE) \
            -smp $(SMP) \
			-cpu $(X86_CPU) \
            -kernel $(KERNEL) \
            $(QEMU_ARGS) \
            -serial mon:stdio \
            -s -S

gdb-client:
	@gdb -ex 'file $(KERNEL)' -ex 'set arch i386:x86-64' -ex 'target remote localhost:1234'
else
gdb-server: domains build sdcard
	@$(QEMU) \
            -M virt \
            -bios default \
            -kernel $(KERNEL) \
            $(QEMU_ARGS) \
            -smp $(SMP) -m $(MEMORY_SIZE) \
            -s -S

gdb-client:
	@riscv64-unknown-elf-gdb -ex 'file $(KERNEL)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'
endif

clean:
	rm -f build/disk/g*
	rm -f build/init/g*
	cargo clean

ifeq ($(ARCH_KIND),x86_64)
kernel_asm:
	@objdump -d $(KERNEL) > kernel.asm
	@vim kernel.asm
	@rm kernel.asm
else
kernel_asm:
	@riscv64-unknown-elf-objdump -d $(KERNEL) > kernel.asm
	@vim kernel.asm
	@rm kernel.asm
endif

check:
	@cargo fmt
	@cargo clippy -p kernel --target $(TARGET_CONFIG)  -- -D warnings

.PHONY:build domains gdb-client gdb-server img sdcard user mount $(FS) fix initrd check run_uintr fake_run_uintr record_run vdso
