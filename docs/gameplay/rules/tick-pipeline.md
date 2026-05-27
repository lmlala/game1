<!--
Project: game1
Created Date: 2026-05-27
Author: liming
Email: lmlala@aliyun.com
Copyright (c) 2025 FiuAI
-->

# Tick 流程规则

**适用场景**: 需要实现 Tick runner、排查模拟顺序或设计回放测试时.

## 固定顺序

```mermaid
flowchart LR
    TickStart[StartTick] --> Economy[EconomySystem]
    Economy --> Decision[DecisionSystem]
    Decision --> Action[ActionExecutionSystem]
    Action --> Combat[CombatSystem]
    Combat --> Relationship[RelationshipSystem]
    Relationship --> EventGen[EventSystem]
    EventGen --> Logging[LoggingSystem]
    Logging --> Metrics[MetricsSnapshot]
```

## 阶段职责

| 阶段 | 输入 | 输出 |
|------|------|------|
| EconomySystem | 帮派、地盘、成员成本 | 收入、支出、赤字、士气事件 |
| DecisionSystem | 角色欲望、性格、状态、关系 | 行动意向事件 |
| ActionExecutionSystem | 玩家指令、行动意向 | 偷窃、奖励、处罚、投奔、进攻等结果事件 |
| CombatSystem | 冲突事件、参战者属性 | 伤亡、胜负、误伤、领地易主事件 |
| RelationshipSystem | 已发生事件 | 记忆、仇恨、好感、恐惧更新事件 |
| EventSystem | 世界状态和近期事件 | 内乱、举报、奇遇、财政崩溃等派生事件 |
| LoggingSystem | 本 Tick 事件 | 可读日志和摘要 |
| MetricsSnapshot | 本 Tick 状态 | 验收指标和调参数据 |

## 确定性要求

- 每局启动必须记录 seed.
- 同 Tick 内遍历实体时使用稳定排序.
- 随机数只从模拟上下文获取, 不直接调用系统随机源.
- 玩家指令在 Tick 开始前排队, 按确定性顺序应用.

## 异常处理

- 无效实体引用: 返回错误事件或命令错误, 不跳过.
- 数值越界: clamp 后记录调试信息, 关键配置越界应失败.
- 死亡角色行动: 拒绝行动意向并记录被过滤原因.
- 帮派灭亡: 进入失效状态, 不从历史事件中删除.

## 不变量

- Tick 编号单调递增.
- 事件顺序稳定.
- 所有状态变更可由事件解释.
- 日志生成失败不得影响结构化事件保存.
