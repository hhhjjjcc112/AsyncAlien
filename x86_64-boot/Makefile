TARGET := x86_64-unknown-none
KERNEL_ELF := ../target/$(TARGET)/release/x86_64-boot
KERNEL_BIN := ../target/$(TARGET)/release/kernel.bin

.PHONY: all build clean run

all: build

build:
	@echo "Building kernel..."
	@cargo build --target $(TARGET) --release
	@echo "Copying ELF file..."
	@rust-objcopy --strip-all --binary-architecture=x86_64 $(KERNEL_ELF) -O binary $(KERNEL_BIN)


dump:
	@echo "Generating assembly dump..."
	@rust-objdump -d --print-imm-hex --x86-asm-syntax=intel $(KERNEL_ELF) > kernel_dump.asm
run: build
	@echo "Starting QEMU..."
	@qemu-system-x86_64 \
		-machine q35 \
		-smp 1 \
		-kernel $(KERNEL_ELF) \
		-nographic \
		-serial mon:stdio

debug: build
	@echo "Starting QEMU with GDB stub..."
	@qemu-system-x86_64 \
		-machine q35 \
		-smp 1 \
		-kernel $(KERNEL_ELF) \
		-nographic \
		-serial mon:stdio \
		-s -S

# 连接到正在运行debug的QEMU实例
gdb: 
	@echo "Starting GDB..."
	@gdb $(KERNEL_ELF) \
		-ex "target remote :1234" \
		-ex "layout asm"

clean:
	@cargo clean
