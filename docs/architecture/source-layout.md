<!--
Project: game1
Created Date: 2026-05-27
Author: liming
Email: lmlala@aliyun.com
Copyright (c) 2025 FiuAI
-->

# 源码目录结构设计

**适用场景**: 需要从文档设计进入 Rust/Godot 实现、拆分模块或新增目录时.

## 总原则

- Rust 是确定性模拟和规则事实源.
- Godot 是 UI、场景、输入和表现层.
- `docs/gameplay/` 描述玩法规则, `docs/simulation/` 描述事件、日志和验收.
- 新代码先对齐文档, 再落到对应模块.

## Rust 目标结构

```text
rust/src/
├── lib.rs              # GDExtension 注册入口
├── core/               # 无 Godot 依赖的纯模拟核心
│   ├── mod.rs
│   ├── world/          # 实体、组件、WorldState、初始化
│   ├── rules/          # 经济、决策、行动、战斗、关系、事件触发
│   ├── events/         # 事件类型、事件总线、事件应用、回放
│   ├── logging/        # 日志模板、渲染、摘要
│   ├── sim/            # Tick runner、seed、命令队列、指标快照
│   └── content/        # V1 初始帮派、地盘、角色模板、文本语料
└── nodes/              # Godot 适配层, 暴露 #[func] 和信号
```

## 模块职责

| 模块 | 职责 | 不应包含 |
|------|------|----------|
| `core/world/` | 数据结构、实体存储、初始化校验 | 日志文案、Godot 类型 |
| `core/rules/` | 规则计算和事件生成 | UI 状态、文件 IO |
| `core/events/` | 事件 schema、排序、应用、回放 | 自然语言模板 |
| `core/logging/` | 从事件渲染可读文本 | 反向修改世界状态 |
| `core/sim/` | Tick 编排、seed、命令队列、指标 | 具体 UI 控件 |
| `core/content/` | Demo V1 数据和模板 | 运行期 mutable 状态 |
| `nodes/` | Godot API 适配 | 复杂规则重复实现 |

## Godot 目标结构

```text
godot/
├── scenes/     # UI 和场景
├── scripts/    # GDScript 薄胶水层
├── assets/     # 美术、音频、字体等资源
└── *.gdextension
```

第一版后端日志 demo 暂不需要新增 Godot UI. 后续接 UI 时, Godot 只负责:

- 发送玩家宏观指令.
- 展示 Tick 日志和指标.
- 订阅 Rust 暴露的模拟状态或信号.

## 实现顺序建议

1. `core/events/`: 事件类型和 schema.
2. `core/world/`: V1 初始世界和实体校验.
3. `core/sim/`: Tick runner 和 seed.
4. `core/rules/`: 经济、决策、行动、战斗、关系.
5. `core/logging/`: 文本日志输出.
6. `nodes/`: Godot 适配, 等后端日志验证成立后再做.

## 文件规模

- 单个 Rust 文件过长时优先按模块拆分.
- 单个函数保持短小, 复杂流程拆成输入校验、规则计算、事件生成、事件应用.
- 新增源码文件必须使用项目文件头.
