# 架构概览

**适用场景**: 需要理解 Godot 与 Rust 职责边界、扩展新系统时.

## 分层

```text
┌─────────────────────────────────────┐
│  godot/  UI · 场景 · 输入 · 表现     │
│  (GDScript 仅作薄胶水, 不写核心规则)  │
├─────────────────────────────────────┤
│  GDExtension  (game1_core.gdextension)│
├─────────────────────────────────────┤
│  rust/   核心逻辑 · 模拟 · 规则      │
│  (game1_core crate, godot-rust)      │
└─────────────────────────────────────┘
```

## 边界原则

| 放在 Godot | 放在 Rust |
|------------|-----------|
| 场景布局、动画、粒子、UI 控件 | 确定性游戏逻辑、数值计算 |
| 资源引用、导出变量展示 | 复杂状态机、存档序列化核心 |
| 调用 Rust 暴露的 `#[func]` API | 可单元测试的纯逻辑 (后续可拆模块) |

## 类注册流程

1. 在 `rust/src/` 用 `#[derive(GodotClass)]` 定义类
2. `cargo build` 生成 `libgame1_core.{dylib,so,dll}`
3. Godot 通过 `game1_core.gdextension` 加载
4. 在编辑器中将节点类型改为 Rust 类 (如 `Player`)

## 数据流 (目标形态)

- **Godot → Rust**: 调用 `#[func]` 方法, 或信号连接到 Rust
- **Rust → Godot**: `#[signal]` 发射, 或 `base_mut()` 操作节点
- 避免在 GDScript 重复实现已在 Rust 存在的规则
