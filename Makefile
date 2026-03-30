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
SMP ?= 2
MEMORY_SIZE := 2048M
X86_CPU ?= Icelake-Server,+x2apic
X86_UINTR_CPU ?= max,+x2apic,+uintr
VIRTIO_FORCE_LEGACY ?= y
LOG ?=
GUI ?=n
FS ?=fat
IMG := build/sdcard.img
FSMOUNT := ./diskfs
FEATURES := default
DOMAIN_PROFILE ?=
INITRD_REBUILD_USER ?= y
name ?=
VF2 ?= n
TFTPBOOT := /home/godones/projects/tftpboot/
VF2_SD ?= n
BUILD_CFG ?=  -Z build-std=core,alloc -Z build-std-features=compiler-builtins-mem
BENCH ?= n
comma:= ,
empty:=
space:= $(empty) $(empty)

QEMU_ARGS :=

ifeq ($(ARCH_KIND),x86_64)
    # x86_64 迁移阶段默认单核启动，避免 AP bring-up 干扰主链路。
    ifneq ($(origin SMP),command line)
        ifneq ($(origin SMP),environment)
            SMP := 1
        endif
    endif
    # x86_64 QEMU args
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
FEATURES += bench
endif

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
	@echo "  make ready              Build everything but don't run QEMU"
	@echo "  make build              Build kernel only"
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
	@echo "  X86_CPU=...             x86 CPU model/features (default: Icelake-Server,+x2apic)"
	@echo "  GUI=y/n                 Enable GUI (default: n)"
	@echo "  LOG=level               Log level"
	@echo "  VF2_SD=y/n              Enable VF2 SD card support (default: n)"
	@echo ""
	@echo "Examples:"
	@echo "  make ARCH=riscv64 PLATFORM=plat_qemu_riscv run"
	@echo "  make ARCH=x86_64 PLATFORM=plat_qemu_x86_64 build SMP=4"
	@echo "  make ARCH=x86_64 run_uintr X86_UINTR_CPU=\"max,+x2apic,+uintr\""
	@echo "  make ARCH=riscv64 PLATFORM=plat_vf2 build VF2_SD=y"

# Build everything but don't run QEMU (same as run without the QEMU step)
ready: domains sdcard initrd build
	@echo "Build complete. Ready to run with 'make fake_run' or 'make run'"

build:
	@echo "Building..."
	@echo "ARCH: $(ARCH)"
	@echo "PLATFORM: $(PLATFORM)"
	@echo "TARGET: $(TARGET)"
	@echo "SMP: $(SMP)"
	@echo "VF2_SD: $(VF2_SD)"
	@#LOG=$(LOG) cargo build --release -p kernel --target $(TARGET) --features $(FEATURES)
	PLATFORM=$(PLATFORM) RUSTFLAGS='--cfg getrandom_backend="custom" --cfg $(PLATFORM)' LOG=$(LOG) cargo build --release -p kernel --target $(TARGET_CONFIG) $(BUILD_CFG) --features $(FEATURES)

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
endif

user:
	@echo "Building user apps"
	@make all -C ./user/apps ARCH=$(ARCH_KIND)
	@make all -C ./user/musl ARCH=$(ARCH_KIND)
	@echo "Building user apps done"


sdcard:$(FS) mount user #domains
	@sudo cp build/disk/* $(FSMOUNT)/
	@-sudo cp user/bin/* $(FSMOUNT)/
	@sudo mkdir -p $(FSMOUNT)/domains
	@sudo ls $(FSMOUNT)
	@sudo umount $(FSMOUNT)
	@rm -rf $(FSMOUNT)

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
	@make initrd

domain:
	cd domains && ARCH=$(ARCH) PLATFORM=$(PLATFORM) DOMAIN_PROFILE=$(DOMAIN_PROFILE) cargo domain build -n $(name) -l "$(LOG)" -o $(abspath build)
	@make initrd

initrd:
	@if [ "$(INITRD_REBUILD_USER)" = "y" ]; then \
		echo "[initrd] build user/initrd ARCH=$(ARCH_KIND)"; \
		if ! make -C user/initrd ARCH=$(ARCH_KIND); then \
			echo "[WARN] user/initrd 构建失败，回退使用现有 initramfs-$(ARCH_KIND)"; \
		fi; \
	else \
		echo "[initrd] skip user/initrd rebuild (INITRD_REBUILD_USER=$(INITRD_REBUILD_USER))"; \
	fi
	@mkdir -p ./initrd
	@cp ./build/init/g* ./initrd
	@rm -f ./initrd/gvirtio_blk ./initrd/gvirtio_net ./initrd/gvirtio_input ./initrd/gvirtio_gpu
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

.PHONY:build domains gdb-client gdb-server img sdcard user mount $(FS) fix initrd check run_uintr fake_run_uintr
