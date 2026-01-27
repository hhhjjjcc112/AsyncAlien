# 在仓库根目录运行（/home/hjch/AsyncAlien）
set -e

for d in utils/*; do
  # 仅处理目录
  [ -d "$d" ] || continue

  # 是否是 git 仓库
  if git -C "$d" rev-parse --git-dir >/dev/null 2>&1; then
    url="$(git -C "$d" remote get-url origin 2>/dev/null || true)"
    if [ -z "$url" ]; then
      echo "跳过：$d 未配置 origin 远程"
      continue
    fi

    echo "处理：$d -> $url"

    # 如果该目录已被父仓库当作普通文件跟踪，先从索引移除（保留工作区）
    if git ls-files --error-unmatch "$d" >/dev/null 2>&1; then
      git rm -r --cached "$d"
    fi

    # 可选：按当前分支跟踪（若远程存在该分支）
    branch="$(git -C "$d" branch --show-current 2>/dev/null || true)"
    if [ -n "$branch" ]; then
      git submodule add -f -b "$branch" "$url" "$d" || git submodule add -f "$url" "$d"
    else
      git submodule add -f "$url" "$d"
    fi

    # 将已有 .git 结构吸收到父仓库的 .git/modules 中
    git submodule absorbgitdirs "$d"
  fi
done

# 初始化并更新子模块
git submodule update --init --recursive

# 提交变更（包含 .gitmodules 与 gitlink）
git add .gitmodules
git add utils/*
git commit -m "Convert utils/* cloned repos into Git submodules"