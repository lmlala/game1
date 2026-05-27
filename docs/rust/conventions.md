# Rust 侧约定

**适用场景**: 编辑 `rust/`、新增 Godot 类、暴露 API 时.

## Crate

- 包名: `game1_core`
- 类型: `cdylib` (GDExtension 动态库)
- 入口: `ExtensionLibrary` + `#[gdextension]` 于 `lib.rs`

## 模块组织 (演进)

```text
rust/src/
├── lib.rs          # gdextension 入口 + mod 声明
├── nodes/          # #[derive(GodotClass)] 节点
└── core/           # 纯逻辑 (无 Godot 依赖, 便于测试)
```

单文件超过 ~400 行时拆模块.

## godot-rust 要点

- `use godot::prelude::*;` 与 `godot::classes::*`
- 引擎回调: `#[godot_api] impl I{Base} for YourClass`
- 暴露给编辑器/GDScript: `#[godot_api] impl YourClass` + `#[func]` / `#[signal]`
- 访问基类: `self.base()` / `self.base_mut()`, 不要直接用 `self.base` 字段

## 构建

```bash
cd rust && cargo build          # debug
cd rust && cargo build --release
```

产物: `rust/target/debug/libgame1_core.dylib` (macOS)

## 工具链

`rust/rust-toolchain.toml` 要求 **rustc >= 1.94** (godot 0.5.x).
