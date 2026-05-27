# game1 文档索引

> AI 编码前先读本文件, 再按任务进入子目录文档.

## 项目一句话

**Godot 4 (`godot/`) 负责 UI/场景; Rust (`rust/`) 通过 GDExtension 承载核心逻辑.**

## 目录地图

| 路径 | 用途 |
|------|------|
| [architecture/overview.md](architecture/overview.md) | 分层、边界、数据流 |
| [godot/conventions.md](godot/conventions.md) | 场景、GDScript、与 Rust 桥接 |
| [rust/conventions.md](rust/conventions.md) | crate 结构、godot-rust 约定 |
| [dev/setup.md](dev/setup.md) | 环境安装与首次跑通 |
| [dev/workflow.md](dev/workflow.md) | 日常开发、热重载、排错 |

## 源码地图

```
game1/
├── godot/          # Godot 工程 (project.godot)
│   ├── scenes/     # .tscn 场景
│   ├── scripts/    # GDScript (薄胶水层)
│   ├── assets/     # 美术/音频等资源
│   └── *.gdextension
├── rust/           # game1_core crate (cdylib)
│   └── src/lib.rs  # ExtensionLibrary + Rust 类
├── docs/           # 本目录
└── scripts/        # 本地辅助脚本
```

## 外部参考

- [godot-rust book](https://godot-rust.github.io/book/)
- [Godot 4 文档](https://docs.godotengine.org/en/stable/)

## 文档维护规则

1. 新主题优先放入已有子目录; 子目录超过 ~5 篇再拆子文件夹
2. 每篇文档文首写「适用场景」一行
3. 架构决策写入 `architecture/`, 操作步骤写入 `dev/`
4. 改目录结构时同步更新本 INDEX
