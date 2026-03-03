## Architecture selection: riscv64 (default), x86_64, or vf2
ARCH ?= riscv64

# Validate ARCH value
VALID_ARCHS := riscv64 x86_64 vf2
ifeq ($(filter $(ARCH),$(VALID_ARCHS)),)
    $(error Invalid ARCH=$(ARCH). Valid values are: $(VALID_ARCHS))
endif

ifeq ($(ARCH),x86_64)
    TARGET := x86_64-unknown-none
    TARGET_CONFIG := ./tools/x86_64.json
    TARGET2 := x86_64
    QEMU := qemu-system-x86_64
    PLATFORM := plat_qemu_x86_64
else ifeq ($(ARCH),vf2)
    TARGET := riscv64gc-unknown-none-elf
    TARGET_CONFIG := ./tools/riscv64.json
    TARGET2 := riscv64
    QEMU := qemu-system-riscv64
    PLATFORM := plat_vf2
else ifeq ($(ARCH),riscv64)
    TARGET := riscv64gc-unknown-none-elf
    TARGET_CONFIG := ./tools/riscv64.json
    TARGET2 := riscv64
    QEMU := qemu-system-riscv64
    PLATFORM := plat_qemu_riscv
endif

PROFILE := release
KERNEL := target/$(TARGET2)/$(PROFILE)/kernel
NET ?= y
SMP ?= 2
MEMORY_SIZE := 2048M
LOG ?=
GUI ?=n
FS ?=fat
IMG := build/sdcard.img
FSMOUNT := ./diskfs
FEATURES := default
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

ifeq ($(ARCH),x86_64)
    # x86_64 QEMU args
    ifeq ($(GUI),y)
        QEMU_ARGS += -device virtio-gpu-pci \
                     -device virtio-keyboard-pci \
                     -device virtio-mouse-pci
    else
        QEMU_ARGS += -nographic
    endif
    ifeq ($(NET),y)
        QEMU_ARGS += -device virtio-net-pci,netdev=net0 \
                     -netdev user,id=net0,hostfwd=tcp::55555-:55555,hostfwd=udp::5555-:5555
    endif
    QEMU_ARGS += -drive file=$(IMG),if=none,format=raw,id=x0 \
                 -device virtio-blk-pci,drive=x0
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
        QEMU_ARGS += -device virtio-net-device,netdev=net0 \
                     -netdev user,id=net0,hostfwd=tcp::55555-:55555,hostfwd=udp::5555-:5555
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
	@echo "  make ARCH=riscv64 ...   Build for RISC-V 64-bit QEMU (default)"
	@echo "  make ARCH=x86_64 ...    Build for x86-64 QEMU"
	@echo "  make ARCH=vf2 ...       Build for VisionFive 2 board"
	@echo ""
	@echo "Main Targets:"
	@echo "  make run                Build and run in QEMU"
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
	@echo "  GUI=y/n                 Enable GUI (default: n)"
	@echo "  LOG=level               Log level"
	@echo "  VF2_SD=y/n              Enable VF2 SD card support (default: n)"
	@echo ""
	@echo "Examples:"
	@echo "  make ARCH=riscv64 run"
	@echo "  make ARCH=x86_64 build SMP=4"
	@echo "  make vf2 VF2_SD=y       Build for VF2 with SD card"

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
	@$(MAKE) ARCH=vf2 build
	rust-objcopy --strip-all target/riscv64/release/kernel -O binary ./testos.bin
	cp ./testos.bin  $(TFTPBOOT)
	rm ./testos.bin

ifeq ($(ARCH),x86_64)
# x86_64 run target
run: domains sdcard initrd build
	$(QEMU) \
            -m $(MEMORY_SIZE) \
            -smp $(SMP) \
            -cpu Icelake-Server,+x2apic \
            -kernel $(KERNEL) \
            $(QEMU_ARGS) \
            -serial mon:stdio

fake_run:
	$(QEMU) \
            -m $(MEMORY_SIZE) \
            -smp $(SMP) \
            -cpu Icelake-Server,+x2apic \
            -kernel $(KERNEL) \
            $(QEMU_ARGS) \
            -serial mon:stdio
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
	@make all -C ./user/apps
	@make all -C ./user/musl
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
	cd domains && cargo domain build-all -l "$(LOG)" -o $(abspath build)
	@make initrd

domain:
	cd domains && cargo domain build -n $(name) -l "$(LOG)" -o $(abspath build)
	@make initrd

initrd:
	@make -C user/initrd
	@mkdir -p ./initrd
	@cp ./build/init/g* ./initrd
	@cp ./user/initrd/initramfs/* ./initrd -r
	@#-cp ./user/bin/* ./initrd/bin -r
	@#cd ./initrd && find . -print0 | cpio --null -ov --format=newc | gzip -9 > ../build/initramfs.cpio.gz && cd ..
	@cd ./initrd && find . | cpio -o -H newc | gzip -9 > ../build/initramfs.cpio.gz && cd ..
	@rm -rf ./initrd


ifeq ($(ARCH),x86_64)
gdb-server: domains build sdcard
	@$(QEMU) \
            -m $(MEMORY_SIZE) \
            -smp $(SMP) \
            -cpu Icelake-Server,+x2apic \
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
	rm build/disk/g*
	rm build/init/g*
	cargo clean

ifeq ($(ARCH),x86_64)
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

.PHONY:build domains gdb-client gdb-server img sdcard user mount $(FS) fix initrd check
