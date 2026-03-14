#!/usr/bin/env bash
set -euo pipefail

# 最小化 x86_64 bring-up：仅编译内核并直接启动 QEMU。
# 用途：迁移中途验证 boot/基础环境/设备发现链路是否通。

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT_DIR"

SMP="${SMP:-2}"
MEMORY_SIZE="${MEMORY_SIZE:-2048M}"
LOG_LEVEL="${LOG:-}"
ENABLE_NET="${NET:-n}"
X86_CPU="${X86_CPU:-max,+x2apic}"
MEMORY_SELF_TEST="${MEMORY_SELF_TEST:-y}"

KERNEL_FEATURES="${FEATURES:-default}"
KERNEL_FEATURES="${KERNEL_FEATURES// /,}"
if [[ "$MEMORY_SELF_TEST" == "y" ]]; then
  # 默认启用内存自检，便于最小化启动时尽早发现问题。
  if [[ ",$KERNEL_FEATURES," != *",memory_self_test,"* ]]; then
    KERNEL_FEATURES="$KERNEL_FEATURES,memory_self_test"
  fi
fi

echo "[minimal-x86] build kernel only"
echo "[minimal-x86] features: $KERNEL_FEATURES"
make build ARCH=x86_64 LOG="$LOG_LEVEL" FEATURES="$KERNEL_FEATURES"

QEMU_ARGS=(
  -m "$MEMORY_SIZE"
  -smp "$SMP"
  -cpu "$X86_CPU"
  -kernel target/x86_64/release/kernel
  -nographic
  -serial mon:stdio
)

if [[ -f build/initramfs.cpio.gz ]]; then
  QEMU_ARGS+=( -initrd build/initramfs.cpio.gz -append "rdinit=/init" )
fi

QEMU_ARGS+=( -drive file=build/sdcard.img,if=none,format=raw,id=x0 )
QEMU_ARGS+=( -device virtio-blk-pci,drive=x0 )

if [[ "$ENABLE_NET" == "y" ]]; then
  QEMU_ARGS+=( -device virtio-net-pci,netdev=net0 )
  QEMU_ARGS+=( -netdev user,id=net0,hostfwd=tcp::55555-:55555 )
fi

echo "[minimal-x86] run qemu"
exec qemu-system-x86_64 "${QEMU_ARGS[@]}"
