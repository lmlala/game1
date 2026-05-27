# game1 — Agent 指南

## 技术栈

- **Godot 4** (`godot/`): 场景、UI、资源
- **Rust** (`rust/`, crate `game1_core`): GDExtension 核心逻辑, [godot-rust](https://godot-rust.github.io/book/) 0.5.x

## 必读文档 (按优先级)

1. [`docs/INDEX.md`](docs/INDEX.md) — 文档总索引
2. 玩法/模拟任务: `docs/gameplay/`, `docs/simulation/`
3. 工程任务: `docs/architecture/`, `docs/godot/`, `docs/rust/`, `docs/dev/`

## 仓库布局

| 路径 | 职责 |
|------|------|
| `godot/scenes/` | `.tscn` |
| `godot/scripts/` | GDScript 薄层 |
| `rust/src/` | Rust 类与核心逻辑 |
| `docs/gameplay/` | 玩法、世界、规则、Demo 边界 |
| `docs/simulation/` | 事件 schema、日志、指标验收 |
| `godot/game1_core.gdextension` | 指向 `../rust/target/` 动态库 |

## 硬性约定

- 核心游戏规则写 **Rust**, 不在 GDScript 重复实现
- 修改 Rust 后需 `cargo build`; 路径与 `.gdextension` 保持一致
- 不提交 `rust/target/`, `godot/.godot/`
- 不修改 `.env` 或硬编码密钥
- 新建源码文件使用项目标准文件头 (见用户规则)
- 玩法规则变更同步更新 `docs/gameplay/`, 事件/日志/验收变更同步更新 `docs/simulation/`

## 构建

```bash
./scripts/build-rust.sh
./scripts/run-godot.sh
```

Rust 工具链: **>= 1.94** (`rust/rust-toolchain.toml`).
