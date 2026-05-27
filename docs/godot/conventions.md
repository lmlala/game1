# Godot 侧约定

**适用场景**: 编辑 `.tscn`、GDScript、资源路径时.

## 目录

| 目录 | 内容 |
|------|------|
| `scenes/` | 场景树, 按功能分子目录 (如 `scenes/ui/`) |
| `scripts/` | GDScript, 仅 UI 事件与 Rust API 调用 |
| `assets/` | 贴图、音频、字体; 按类型分子目录 |

## 命名

- 场景: `snake_case.tscn` (如 `main_menu.tscn`)
- 节点: PascalCase 与 Godot 习惯一致
- GDScript: `snake_case.gd`

## Rust 节点挂载

1. 场景中放置与 Rust `base` 一致的节点 (如 `Sprite2D`)
2. 右键 → **Change Type...** → 选择 Rust 类 (如 `Player`)
3. 若类未出现: 先 `cargo build`, 重启编辑器或依赖 `reloadable`

## 禁止

- 在 GDScript 实现核心游戏规则 (应下沉到 Rust)
- 修改 `../rust/target/` 内产物
- 提交 `.godot/` 缓存 (已在 `.gitignore`)
