# game1 文档索引

> AI 编码前先读本文件, 再按任务进入子目录文档.

## 项目一句话

**Godot 4 (`godot/`) 负责 UI/场景; Rust (`rust/`) 通过 GDExtension 承载核心逻辑.**

## 目录地图

| 路径 | 用途 |
|------|------|
| [architecture/overview.md](architecture/overview.md) | 分层、边界、数据流 |
| [architecture/source-layout.md](architecture/source-layout.md) | Rust/Godot 未来源码目录结构 |
| [gameplay/overview.md](gameplay/overview.md) | 玩法目标、MVP 边界、玩家爽点 |
| [gameplay/demo-v1.md](gameplay/demo-v1.md) | 第一版日志驱动涌现 demo |
| [gameplay/world.md](gameplay/world.md) | 帮派、地盘、角色模板、世界观语调 |
| [gameplay/rules/entities.md](gameplay/rules/entities.md) | Person/Gang/Territory/Event 实体规则 |
| [gameplay/rules/tick-pipeline.md](gameplay/rules/tick-pipeline.md) | Tick 顺序、不变量、异常处理 |
| [gameplay/rules/actions.md](gameplay/rules/actions.md) | 玩家指令与角色自动行动规则 |
| [gameplay/rules/events.md](gameplay/rules/events.md) | V1 事件目录、触发来源、日志优先级 |
| [gameplay/rules/drama-amplifiers.md](gameplay/rules/drama-amplifiers.md) | 财政、奖惩、误伤、复仇等戏剧放大器 |
| [simulation/event-schema.md](simulation/event-schema.md) | 事件 schema、事件事实源 |
| [simulation/logging.md](simulation/logging.md) | 日志模板、优先级、渲染规则 |
| [simulation/metrics.md](simulation/metrics.md) | 后端日志验收指标 |
| [godot/conventions.md](godot/conventions.md) | 场景、GDScript、与 Rust 桥接 |
| [rust/conventions.md](rust/conventions.md) | crate 结构、godot-rust 约定 |
| [dev/setup.md](dev/setup.md) | 环境安装与首次跑通 |
| [dev/workflow.md](dev/workflow.md) | 日常开发、热重载、排错 |

## 源码地图

```text
game1/
├── godot/          # Godot 工程 (project.godot)
│   ├── scenes/     # .tscn 场景
│   ├── scripts/    # GDScript (薄胶水层)
│   ├── assets/     # 美术/音频等资源
│   └── *.gdextension
├── rust/           # game1_core crate (cdylib)
│   └── src/lib.rs  # ExtensionLibrary + Rust 类
├── docs/           # 本目录
│   ├── gameplay/   # 玩法、世界、规则、Demo 边界
│   └── simulation/ # 事件、日志、指标
└── scripts/        # 本地辅助脚本
```

## 外部参考

- [godot-rust book](https://godot-rust.github.io/book/)
- [Godot 4 文档](https://docs.godotengine.org/en/stable/)

## 文档维护规则

1. 新主题优先放入已有子目录; 子目录超过 ~5 篇再拆子文件夹
2. 每篇文档文首写「适用场景」一行
3. 架构决策写入 `architecture/`, 玩法规则写入 `gameplay/`, 事件日志验收写入 `simulation/`, 操作步骤写入 `dev/`
4. 改目录结构时同步更新本 INDEX 与 `AGENTS.md`
