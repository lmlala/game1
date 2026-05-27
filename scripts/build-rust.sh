#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "${ROOT}/rust"
cargo build "$@"
mkdir -p "${ROOT}/godot/bin"
if [[ "$(uname -s)" == "Darwin" ]]; then
  cp -f target/debug/libgame1_core.dylib "${ROOT}/godot/bin/"
  echo "已复制: godot/bin/libgame1_core.dylib"
else
  cp -f target/debug/libgame1_core.so "${ROOT}/godot/bin/"
  echo "已复制: godot/bin/libgame1_core.so"
fi
echo "请完全退出并重启 Godot 编辑器后再运行游戏."
