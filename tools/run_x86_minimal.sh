#!/usr/bin/env bash
set -euo pipefail

# 最小化 x86_64 bring-up：仅编译内核并直接启动 QEMU。
# 用途：迁移中途验证 boot/基础环境/设备发现链路是否通。

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT_DIR"

SMP="${SMP:-1}"
MEMORY_SIZE="${MEMORY_SIZE:-2048M}"
LOG_LEVEL="${LOG:-DEBUG}"
ENABLE_NET="${NET:-n}"
WITH_INITRD="${WITH_INITRD:-y}"
WITH_DRIVE="${WITH_DRIVE:-y}"
WITH_NET="${WITH_NET:-$ENABLE_NET}"
WITH_INPUT="${WITH_INPUT:-n}"
WITH_GPU="${WITH_GPU:-n}"
VIRTIO_FORCE_LEGACY="${VIRTIO_FORCE_LEGACY:-y}"
NET_HOSTFWD="${NET_HOSTFWD:-y}"
NET_FWD_PORT="${NET_FWD_PORT:-55555}"
X86_MACHINE="${X86_MACHINE:-q35}"
X86_ACCEL="${X86_ACCEL:-auto}"
if [[ "$X86_ACCEL" == "auto" ]]; then
  if [[ -r /dev/kvm && -w /dev/kvm ]]; then
    X86_ACCEL="kvm"
  else
    X86_ACCEL="tcg"
  fi
fi
if [[ "$X86_ACCEL" == "kvm" && ! -r /dev/kvm ]]; then
  echo "[minimal-x86] /dev/kvm unavailable, fallback to tcg"
  X86_ACCEL="tcg"
fi
if [[ -z "${X86_CPU:-}" ]]; then
  if [[ "$X86_ACCEL" == "kvm" ]]; then
    X86_CPU="host,+x2apic"
  else
    X86_CPU="max"
  fi
fi
APIC_TIMER_TEST="${APIC_TIMER_TEST:-n}"
UNWIND_TEST="${UNWIND_TEST:-n}"
MEMORY_TEST="${MEMORY_TEST:-y}"
TRAP_TEST="${TRAP_TEST:-y}"
DOMAIN_TEST="${DOMAIN_TEST:-n}"
DOMAIN_SYSCALL_TEST="${DOMAIN_SYSCALL_TEST:-n}"
DOMAIN_TASK_TEST="${DOMAIN_TASK_TEST:-n}"
DOMAIN_APIC_TEST="${DOMAIN_APIC_TEST:-n}"
DOMAIN_UART_TEST="${DOMAIN_UART_TEST:-n}"
DOMAIN_BLOCK_TEST="${DOMAIN_BLOCK_TEST:-n}"
DOMAIN_NET_TEST="${DOMAIN_NET_TEST:-n}"

KERNEL_FEATURES="${FEATURES:-default}"
KERNEL_FEATURES="${KERNEL_FEATURES// /,}"
if [[ "$APIC_TIMER_TEST" == "y" ]]; then
  if [[ ",$KERNEL_FEATURES," != *",apic_timer_test,"* ]]; then
    KERNEL_FEATURES="$KERNEL_FEATURES,apic_timer_test"
  fi
fi

if [[ "$UNWIND_TEST" == "y" ]]; then
  if [[ ",$KERNEL_FEATURES," != *",unwind_test,"* ]]; then
    KERNEL_FEATURES="$KERNEL_FEATURES,unwind_test"
  fi
fi

if [[ "$MEMORY_TEST" == "y" ]]; then
  # 默认启用内存自检，便于最小化启动时尽早发现问题。
  if [[ ",$KERNEL_FEATURES," != *",memory_test,"* ]]; then
    KERNEL_FEATURES="$KERNEL_FEATURES,memory_test"
  fi
fi

if [[ "$TRAP_TEST" == "y" ]]; then
  # 按需启用 trap 自检，避免默认启动行为变化。
  if [[ ",$KERNEL_FEATURES," != *",trap_test,"* ]]; then
    KERNEL_FEATURES="$KERNEL_FEATURES,trap_test"
  fi
fi

if [[ "$DOMAIN_TEST" == "y" ]]; then
  # 域自检默认关闭，仅在验证场景按需开启。
  if [[ ",$KERNEL_FEATURES," != *",domain_test,"* ]]; then
    KERNEL_FEATURES="$KERNEL_FEATURES,domain_test"
  fi
fi

if [[ "$DOMAIN_SYSCALL_TEST" == "y" ]]; then
  if [[ ",$KERNEL_FEATURES," != *",domain_syscall_test,"* ]]; then
    KERNEL_FEATURES="$KERNEL_FEATURES,domain_syscall_test"
  fi
fi

if [[ "$DOMAIN_TASK_TEST" == "y" ]]; then
  if [[ ",$KERNEL_FEATURES," != *",domain_task_test,"* ]]; then
    KERNEL_FEATURES="$KERNEL_FEATURES,domain_task_test"
  fi
fi

if [[ "$DOMAIN_APIC_TEST" == "y" ]]; then
  if [[ ",$KERNEL_FEATURES," != *",domain_apic_test,"* ]]; then
    KERNEL_FEATURES="$KERNEL_FEATURES,domain_apic_test"
  fi
fi

if [[ "$DOMAIN_UART_TEST" == "y" ]]; then
  if [[ ",$KERNEL_FEATURES," != *",domain_uart_test,"* ]]; then
    KERNEL_FEATURES="$KERNEL_FEATURES,domain_uart_test"
  fi
fi

if [[ "$DOMAIN_BLOCK_TEST" == "y" ]]; then
  if [[ ",$KERNEL_FEATURES," != *",domain_block_test,"* ]]; then
    KERNEL_FEATURES="$KERNEL_FEATURES,domain_block_test"
  fi
fi

if [[ "$DOMAIN_NET_TEST" == "y" ]]; then
  if [[ ",$KERNEL_FEATURES," != *",domain_net_test,"* ]]; then
    KERNEL_FEATURES="$KERNEL_FEATURES,domain_net_test"
  fi
fi

echo "[minimal-x86] build kernel only"
echo "[minimal-x86] features: $KERNEL_FEATURES"
echo "[minimal-x86] log level: $LOG_LEVEL"
echo "[minimal-x86] qemu machine: $X86_MACHINE accel: $X86_ACCEL cpu: $X86_CPU"
echo "[minimal-x86] switches: initrd=$WITH_INITRD drive=$WITH_DRIVE net=$WITH_NET input=$WITH_INPUT gpu=$WITH_GPU"
make build ARCH=x86_64 LOG="$LOG_LEVEL" FEATURES="$KERNEL_FEATURES"

VIRTIO_PCI_OPTS=""
if [[ "$VIRTIO_FORCE_LEGACY" == "y" ]]; then
  VIRTIO_PCI_OPTS=",disable-modern=on,disable-legacy=off,x-disable-pcie=on"
fi

QEMU_ARGS=(
  -machine "$X86_MACHINE,accel=$X86_ACCEL"
  -m "$MEMORY_SIZE"
  -smp "$SMP"
  -cpu "$X86_CPU"
  -kernel target/x86_64/release/kernel
  -nographic
  -serial mon:stdio
)

if [[ "$WITH_INITRD" == "y" && -f build/initramfs.cpio.gz ]]; then
  QEMU_ARGS+=( -initrd build/initramfs.cpio.gz -append "rdinit=/init" )
fi

if [[ "$WITH_DRIVE" == "y" ]]; then
  QEMU_ARGS+=( -drive file=build/sdcard.img,if=none,format=raw,id=x0 )
  QEMU_ARGS+=( -device "virtio-blk-pci,drive=x0${VIRTIO_PCI_OPTS}" )
fi

if [[ "$WITH_NET" == "y" ]]; then
  QEMU_ARGS+=( -device "virtio-net-pci,netdev=net0${VIRTIO_PCI_OPTS}" )
  if [[ "$NET_HOSTFWD" == "y" ]]; then
    QEMU_ARGS+=( -netdev "user,id=net0,hostfwd=tcp::${NET_FWD_PORT}-:${NET_FWD_PORT}" )
  else
    QEMU_ARGS+=( -netdev user,id=net0 )
  fi
fi

if [[ "$WITH_INPUT" == "y" ]]; then
  QEMU_ARGS+=( -device "virtio-keyboard-pci${VIRTIO_PCI_OPTS}" )
  QEMU_ARGS+=( -device "virtio-mouse-pci${VIRTIO_PCI_OPTS}" )
fi

if [[ "$WITH_GPU" == "y" ]]; then
  QEMU_ARGS+=( -device "virtio-gpu-pci${VIRTIO_PCI_OPTS}" )
fi

echo "[minimal-x86] run qemu"
exec qemu-system-x86_64 "${QEMU_ARGS[@]}"
