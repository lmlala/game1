# 环境搭建

**适用场景**: 新机器首次 clone 后跑通项目.

## 依赖

| 工具 | 版本建议 |
|------|----------|
| Godot | 4.1+ (本机验证 4.6.2) |
| Rust | stable, **>= 1.94** |
| cargo | 随 rustup |

## 安装 Rust (若版本过低)

```bash
rustup update stable
rustc --version   # 应 >= 1.94
```

## 构建扩展

```bash
./scripts/build-rust.sh
# 或
cd rust && cargo build
```

确认存在: `rust/target/debug/libgame1_core.dylib` (macOS).

## 打开 Godot 工程

```bash
./scripts/run-godot.sh
# 或指定二进制
GODOT_BIN=/path/to/Godot ./scripts/run-godot.sh
```

在项目管理器中打开目录: **`godot/`** (含 `project.godot`).

## 验证

1. 编辑器无 GDExtension 加载错误
2. 打开 `scenes/main.tscn`, F5 运行
3. (可选) 将 `Sprite2D` 改为 `Player` 类型, 应看到旋转与控制台 `Player initialized`

## 常见问题

| 现象 | 处理 |
|------|------|
| 找不到 dylib/so | 检查 `game1_core.gdextension` 路径是否为 `res://../rust/target/...` |
| rustc 版本不够 | `rustup update stable` |
| Rust 类不在 Change Type 列表 | 先 build, 重启 Godot; 查看 Output 面板错误 |
