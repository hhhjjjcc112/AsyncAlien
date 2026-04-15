VERSION = 1.36.1
BB = busybox
ARCH ?= riscv64

COMPILER_riscv64 = riscv64-linux-musl-
COMPILER_x86_64 = x86_64-linux-musl-
COMPILER = $(COMPILER_$(ARCH))

INITRD_DIR = initramfs-$(ARCH)
INITRD_CPIO = initramfs-$(ARCH).cpio.gz
BB_WORK = $(abspath $(BB)-build-$(ARCH))
BB_INSTALL = $(BB_WORK)/_install
BB_WORK_MARK = $(BB_WORK)/.alien_busybox_worktree

EXPECTED_ARCH_riscv64 = RISC-V
EXPECTED_ARCH_x86_64 = x86-64
EXPECTED_ARCH = $(EXPECTED_ARCH_$(ARCH))

ifeq ($(strip $(COMPILER)),)
$(error Unsupported ARCH=$(ARCH), expected riscv64 or x86_64)
endif

all: download build initramfs

download:
	@if [ -d $(BB) ]; then \
		echo "Busybox exists"; \
	else \
		echo "Busybox does not exist"; \
		wget http://busybox.net/downloads/busybox-$(VERSION).tar.bz2; \
		tar -xvf busybox-$(VERSION).tar.bz2 && mv busybox-$(VERSION) $(BB); \
	fi

config:
	@if [ -d $(BB_WORK) ] && [ ! -f $(BB_WORK)/.alien_busybox_worktree ]; then \
		echo "Recreating legacy busybox build dir: $(BB_WORK)"; \
		rm -rf $(BB_WORK); \
	fi
	@if [ ! -d $(BB_WORK) ]; then \
		cp -a $(BB) $(BB_WORK); \
		touch $(BB_WORK)/.alien_busybox_worktree; \
	fi
	CROSS_COMPILE=$(COMPILER) ARCH=$(ARCH) make defconfig -C $(BB_WORK)
	CROSS_COMPILE=$(COMPILER) ARCH=$(ARCH) make menuconfig -C $(BB_WORK)

# 若目标架构尚未配置，则先生成默认配置。
prepare-config:
	@if [ -d $(BB_WORK) ] && [ ! -f $(BB_WORK)/.alien_busybox_worktree ]; then \
		echo "Recreating legacy busybox build dir: $(BB_WORK)"; \
		rm -rf $(BB_WORK); \
	fi
	@if [ ! -d $(BB_WORK) ]; then \
		echo "Creating isolated busybox tree for $(ARCH)"; \
		cp -a $(BB) $(BB_WORK); \
		touch $(BB_WORK)/.alien_busybox_worktree; \
	fi
	@if [ ! -f $(BB_WORK)/.config ]; then \
		echo "Generating default busybox config for $(ARCH)"; \
		CROSS_COMPILE=$(COMPILER) ARCH=$(ARCH) make defconfig -C $(BB_WORK); \
		CROSS_COMPILE=$(COMPILER) ARCH=$(ARCH) make menuconfig -C $(BB_WORK); \
	fi

build: prepare-config
	CROSS_COMPILE=$(COMPILER) ARCH=$(ARCH) make -j $(shell nproc) -C $(BB_WORK)
	CROSS_COMPILE=$(COMPILER) ARCH=$(ARCH) make install -C $(BB_WORK)
	@file $(BB_WORK)/busybox | grep -q "$(EXPECTED_ARCH)" || \
		( echo "busybox arch mismatch: expect $(EXPECTED_ARCH)"; exit 1 )

initramfs:
	@rm -rf $(INITRD_DIR)
	@mkdir -p $(INITRD_DIR)
	cp -r $(BB_INSTALL)/* $(INITRD_DIR)
	@if [ ! -f $(INITRD_DIR)/init ]; then \
		echo "#!/bin/sh\nmount -t proc none /proc\nmount -t sysfs none /sys\nmount -t devtmpfs none /dev\nexec /bin/sh" > $(INITRD_DIR)/init; \
		chmod +x $(INITRD_DIR)/init; \
	fi
	cd $(INITRD_DIR) && find . -print0 | cpio --null -ov --format=newc | gzip -9 > ../$(INITRD_CPIO)
	@ln -snf $(INITRD_CPIO) initramfs.cpio.gz
	@ln -snf $(INITRD_DIR) initramfs

clean:
	@if [ -d $(BB_WORK) ]; then \
		CROSS_COMPILE=$(COMPILER) ARCH=$(ARCH) make clean -C $(BB_WORK); \
	fi
	rm -rf $(INITRD_DIR)
	rm -f $(INITRD_CPIO)

.PHONY: all clean initramfs build config download prepare-config
