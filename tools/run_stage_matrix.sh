#!/usr/bin/env bash
set -euo pipefail

# x86_64 分阶段回归脚本。
# 用法:
#   tools/run_stage_matrix.sh stage0
#   tools/run_stage_matrix.sh all
#   tools/run_stage_matrix.sh riscv-min
#   tools/run_stage_matrix.sh ab-stage4

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT_DIR"

MODE="${1:-stage4}"
TIMEOUT_SEC="${TIMEOUT_SEC:-120}"
LOG_FILE="${LOG_FILE:-run.txt}"
LOG_DIR="${LOG_DIR:-stage_logs}"
mkdir -p "$LOG_DIR"

copy_last_log() {
  local src="$1"
  cp -f "$src" "$LOG_FILE"
}

repack_initrd() {
  local arch_kind="${1:-x86_64}"
  local initramfs_dir="user/initrd/initramfs-${arch_kind}"
  if [[ ! -d "$initramfs_dir" ]]; then
    initramfs_dir="user/initrd/initramfs"
  fi

  rm -rf ./initrd
  mkdir -p ./initrd

  shopt -s nullglob
  local init_bins=(./build/init/g*)
  shopt -u nullglob
  if [[ ${#init_bins[@]} -eq 0 ]]; then
    echo "[stage-matrix] no domain binaries under build/init, skip repack"
    return 1
  fi
  cp "${init_bins[@]}" ./initrd

  if [[ ! -d "$initramfs_dir" ]]; then
    echo "[stage-matrix] missing initramfs dir: $initramfs_dir"
    return 1
  fi
  cp -r "$initramfs_dir"/* ./initrd
  (cd ./initrd && find . | cpio -o -H newc | gzip -9 > ../build/initramfs.cpio.gz)
  rm -rf ./initrd
  return 0
}

prepare_x86_stage() {
  local stage="$1"
  echo "[stage-matrix] build domains (DOMAIN_PROFILE=${stage})"
  set +e
  make domains ARCH=x86_64 PLATFORM=plat_qemu_x86_64 DOMAIN_PROFILE="$stage"
  local rc=$?
  set -e
  if [[ $rc -ne 0 ]]; then
    echo "[stage-matrix] make domains failed(rc=${rc}), fallback repack initrd"
  fi
  repack_initrd "x86_64" || true
}

stage_switches() {
  local stage="$1"
  case "$stage" in
    stage0)
      echo "WITH_DRIVE=n WITH_NET=n WITH_INPUT=n WITH_GPU=n"
      ;;
    stage1)
      echo "WITH_DRIVE=y WITH_NET=y WITH_INPUT=y WITH_GPU=y"
      ;;
    stage2)
      echo "WITH_DRIVE=y WITH_NET=y WITH_INPUT=n WITH_GPU=n"
      ;;
    stage3)
      echo "WITH_DRIVE=y WITH_NET=y WITH_INPUT=y WITH_GPU=n"
      ;;
    stage4)
      echo "WITH_DRIVE=y WITH_NET=y WITH_INPUT=y WITH_GPU=y"
      ;;
    *)
      echo "WITH_DRIVE=y WITH_NET=y WITH_INPUT=y WITH_GPU=y"
      ;;
  esac
}

check_keywords() {
  local stage="$1"
  local stage_log="$2"
  local switches="$3"
  local ok=1

  grep -q "main_entry" "$stage_log" || ok=0

  if [[ "$switches" == *"WITH_DRIVE=y"* ]]; then
    grep -q "<attach domain>: virtio_block" "$stage_log" || ok=0
  fi
  if [[ "$switches" == *"WITH_NET=y"* ]]; then
    grep -q "<attach domain>: virtio_net" "$stage_log" || ok=0
  fi
  if [[ "$switches" == *"WITH_GPU=y"* ]]; then
    grep -q "<attach domain>: virtio_gpu" "$stage_log" || ok=0
    grep -q "<attach domain>: gpu" "$stage_log" || ok=0
    if grep -q "gpu domain not found" "$stage_log"; then ok=0; fi
  fi
  if [[ "$switches" == *"WITH_INPUT=y"* ]]; then
    grep -q "<attach domain>: virtio_input" "$stage_log" || ok=0
    grep -q "<attach domain>: keyboard" "$stage_log" || ok=0
    grep -q "<attach domain>: mouse" "$stage_log" || ok=0
    if grep -q "keyboard domain not found" "$stage_log"; then ok=0; fi
    if grep -q "mouse domain not found" "$stage_log"; then ok=0; fi
  fi

  if [[ "$stage" == "stage4" ]]; then
    grep -q "<attach domain>: block" "$stage_log" || ok=0
    grep -q "<attach domain>: nic" "$stage_log" || ok=0
  fi

  return $((1 - ok))
}

run_one_stage() {
  local stage="$1"
  local stage_log="$LOG_DIR/${stage}.log"

  echo "[stage-matrix] ===== ${stage} ====="
  prepare_x86_stage "$stage"

  local switches
  switches="$(stage_switches "$stage")"

  echo "[stage-matrix] run qemu with timeout=${TIMEOUT_SEC}s"
  echo "[stage-matrix] switches: ${switches} NET_HOSTFWD=n"
  rm -f "$stage_log"

  set +e
  # shellcheck disable=SC2086
  timeout "${TIMEOUT_SEC}"s env \
    DOMAIN_PROFILE="$stage" \
    NET_HOSTFWD=n \
    $switches \
    tools/run_x86_minimal.sh >"$stage_log" 2>&1
  local run_rc=$?
  set -e

  copy_last_log "$stage_log"

  if [[ $run_rc -ne 0 && $run_rc -ne 124 ]]; then
    echo "[stage-matrix] ${stage} run failed, rc=${run_rc}"
    return 1
  fi

  if check_keywords "$stage" "$stage_log" "$switches"; then
    echo "[stage-matrix] ${stage} PASS"
    return 0
  fi

  echo "[stage-matrix] ${stage} FAIL (see ${stage_log})"
  return 1
}

run_ab_stage4() {
  local stage="stage4"
  local failed=0

  echo "[stage-matrix] ===== A/B ${stage} ====="
  prepare_x86_stage "$stage"

  run_ab_case() {
    local tag="$1"
    local force_legacy="$2"
    local with_drive="$3"
    local with_net="$4"
    local expected_marker="$5"
    local ab_log="$LOG_DIR/${stage}_${tag}.log"
    rm -f "$ab_log"
    set +e
    timeout "${TIMEOUT_SEC}"s env \
      DOMAIN_PROFILE="$stage" \
      VIRTIO_FORCE_LEGACY="$force_legacy" \
      WITH_DRIVE="$with_drive" WITH_NET="$with_net" WITH_INPUT=n WITH_GPU=n \
      NET_HOSTFWD=n \
      tools/run_x86_minimal.sh >"$ab_log" 2>&1
    local rc=$?
    set -e
    if [[ $rc -ne 0 && $rc -ne 124 ]]; then
      echo "[stage-matrix] ${tag} run failed, rc=${rc}"
      failed=1
      return
    fi
    if ! grep -q "$expected_marker" "$ab_log"; then
      echo "[stage-matrix] missing marker(${tag}): $expected_marker"
      failed=1
    fi
  }

  echo "[stage-matrix] run blk legacy/modern"
  run_ab_case "blk_legacy" "y" "y" "n" "virtio_blk_pci(legacy)"
  run_ab_case "blk_modern" "n" "y" "n" "virtio_blk_pci(modern)"

  echo "[stage-matrix] run net legacy/modern"
  run_ab_case "net_legacy" "y" "n" "y" "virtio_net_pci(legacy)"
  run_ab_case "net_modern" "n" "n" "y" "virtio_net_pci(modern)"

  copy_last_log "$LOG_DIR/${stage}_net_modern.log"

  if [[ $failed -eq 0 ]]; then
    echo "[stage-matrix] A/B PASS"
    return 0
  fi
  echo "[stage-matrix] A/B FAIL (see ${LOG_DIR}/${stage}_*.log)"
  return 1
}

run_riscv_min() {
  local stage_log="$LOG_DIR/riscv-min.log"

  echo "[stage-matrix] ===== riscv-min ====="
  rm -f "$stage_log"

  echo "[stage-matrix] build riscv domains"
  set +e
  make domains ARCH=riscv64 PLATFORM=plat_qemu_riscv DOMAIN_PROFILE=stage4 >>"$stage_log" 2>&1
  local domains_rc=$?
  set -e
  if [[ $domains_rc -ne 0 ]]; then
    echo "[stage-matrix] riscv domains build failed(rc=${domains_rc}), fallback repack initrd"
    repack_initrd "riscv64" || true
  fi

  echo "[stage-matrix] build riscv kernel"
  set +e
  make build ARCH=riscv64 PLATFORM=plat_qemu_riscv >>"$stage_log" 2>&1
  local build_rc=$?
  set -e
  if [[ $build_rc -ne 0 ]]; then
    echo "[stage-matrix] riscv kernel build failed, rc=${build_rc}"
    copy_last_log "$stage_log"
    return 1
  fi

  # 仅确保镜像存在，避免 fake_run 直接失败。
  set +e
  make sdcard ARCH=riscv64 PLATFORM=plat_qemu_riscv >>"$stage_log" 2>&1
  set -e

  echo "[stage-matrix] run riscv qemu"
  set +e
  timeout "${TIMEOUT_SEC}"s make fake_run ARCH=riscv64 PLATFORM=plat_qemu_riscv GUI=n NET=y NET_HOSTFWD=n >>"$stage_log" 2>&1
  local run_rc=$?
  set -e

  copy_last_log "$stage_log"

  if [[ $run_rc -ne 0 && $run_rc -ne 124 ]]; then
    echo "[stage-matrix] riscv-min failed, rc=${run_rc}"
    return 1
  fi
  if grep -q "Load domains done" "$stage_log" || grep -q "main_entry" "$stage_log"; then
    echo "[stage-matrix] riscv-min PASS"
    return 0
  fi
  echo "[stage-matrix] riscv-min FAIL (see ${stage_log})"
  return 1
}

if [[ "$MODE" == "all" ]]; then
  failed=0
  for stage in stage0 stage1 stage2 stage3 stage4; do
    if ! run_one_stage "$stage"; then
      failed=1
    fi
  done
  if ! run_riscv_min; then
    failed=1
  fi
  exit $failed
fi

if [[ "$MODE" == "riscv-min" ]]; then
  run_riscv_min
  exit 0
fi

if [[ "$MODE" == "ab-stage4" ]]; then
  run_ab_stage4
  exit 0
fi

run_one_stage "$MODE"
