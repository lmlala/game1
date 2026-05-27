<!--
Project: game1
Created Date: 2026-05-27
Author: liming
Email: lmlala@aliyun.com
Copyright (c) 2025 FiuAI
-->

# 事件目录规则

**适用场景**: 需要新增事件类型、调整触发条件或检查日志覆盖时.

## V1 主要事件类型

| 类型 | 触发来源 | 状态影响 | 日志优先级 |
|------|----------|----------|------------|
| `EconomySettled` | 每 Tick 经济结算 | money, morale | minor |
| `FinancialCrisis` | 连续赤字或 money 过低 | morale, recruit 权重 | major |
| `PersonRecruited` | 招募成功 | member_ids | normal |
| `PersonLeftGang` | 流失、驱逐、逃跑 | member_ids, relationship | major |
| `PersonDefected` | 投奔敌帮 | 双方成员和仇恨 | critical |
| `MoneyStolen` | 偷窃行动 | money, memory | major |
| `PersonPunished` | 玩家处罚或帮规 | status, relationship | major |
| `PersonRewarded` | 玩家奖励 | money, loyalty, jealousy | normal |
| `DuelStarted` | 冲动或复仇 | combat queue | major |
| `PersonInjured` | 战斗或事故 | hp, status | normal |
| `PersonKilled` | 战斗、暗杀、反噬 | alive, memory | critical |
| `BattleStarted` | 争地或挑衅 | combat queue | major |
| `BattleResolved` | 战斗结束 | casualties, morale | critical |
| `TerritoryConquered` | 战斗胜利 | controller_gang_id | critical |
| `RelationshipChanged` | 事件后处理 | relationship | minor |
| `MemoryAdded` | 事件后处理 | memory | minor |
| `OfficialReported` | 举报官府 | fine, wanted, revenue | major |
| `AbsurdAccident` | 戏剧放大器 | 伤病、误伤、名声 | major |
| `RumorSpread` | 重大事件后 | influence, relationship | normal |
| `GangDefeated` | 无地盘或无有效成员 | gang state | critical |

## 事件设计要求

- 每个事件定义触发条件、参与者、目标、payload 字段、状态影响和日志模板.
- 关键事件必须能被测试断言, 例如胜负、金额、伤亡、地盘归属.
- 派生事件必须保留原因引用, 例如 `caused_by_event_id`.
- 不新增只用于文本装饰且无状态意义的事件.

## 日志覆盖

- `critical` 和 `major` 默认进入玩家日志.
- `normal` 可按 Tick 摘要合并.
- `minor` 主要用于回放、调试和指标, 默认不逐条展示.
