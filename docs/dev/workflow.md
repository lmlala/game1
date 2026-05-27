# 日常开发流程

**适用场景**: 改 Rust 或 Godot 后的常规迭代.

## Rust 改动

```bash
cd rust && cargo build
```

Godot 4.2+ 且 `reloadable = true` 时, 编辑器失焦再聚焦可热重载扩展.

## Godot 改动

在编辑器内保存场景即可; 版本管理注意 diff `.tscn`.

## 推荐循环

1. 核心逻辑在 `rust/src/core/` 写纯函数 + 单元测试 (后续添加)
2. 在 `rust/src/nodes/` 写 Godot 节点适配层
3. 场景里挂节点, GDScript 只连信号/调 `#[func]`

## 命令速查

```bash
./scripts/build-rust.sh
./scripts/build-rust.sh --release
./scripts/run-godot.sh                    # 打开编辑器
./scripts/run-godot.sh --path godot --headless --quit-after 2  # CI 烟雾
```

## 提交前检查

- [ ] `cargo build` 通过
- [ ] Godot 能加载扩展且无报错
- [ ] 未提交 `rust/target/`, `godot/.godot/`
