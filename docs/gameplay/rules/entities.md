<!--
Project: game1
Created Date: 2026-05-27
Author: liming
Email: lmlala@aliyun.com
Copyright (c) 2025 FiuAI
-->

# 实体与组件规则

**适用场景**: 需要设计 Rust 核心数据结构、初始化世界或校验实体引用时.

## ID 规则

- 所有实体使用稳定 ID, 不依赖显示名称做逻辑关联.
- 事件中引用实体必须使用 ID, 日志渲染阶段再查显示名.
- 指令输入的 ID 必须校验存在性、类型和权限.

## Person

必备字段:

- `id`, `name`, `gang_id`, `role`.
- `combat`: hp, attack, defense.
- `desires`: greed, fame, lust, power, revenge, survival.
- `personality`: impulse, loyalty, suspicion, cowardice, vanity.
- `status`: drunk, wounded, shamed, debt, sick 等标签及持续时间.
- `memory`: 近期关键事件引用和情绪权重.
- `relationships`: 对其他 Person 的关系边.

数值约束:

- 欲望、性格强度、关系强度默认范围为 `0.0..=1.0`.
- 仇恨/好感可用有符号值时必须在 `-1.0..=1.0`.
- hp 不得低于 0; 死亡或失效必须由事件表达.

## Gang

必备字段:

- `id`, `name`, `leader_id`, `member_ids`.
- `money`, `influence`, `morale`.
- `strategy`: expand, defend, recover 等 Tick 倾向.
- `territory_ids`.

不变量:

- leader_id 必须指向本帮有效角色.
- money 可短期为负, 但必须触发财政压力事件.
- 成员变更必须产生招募、流失、投奔、死亡或驱逐事件.

## Territory

必备字段:

- `id`, `name`, `controller_gang_id`.
- `revenue`, `defense`, `risk_tags`.
- `event_tags`: gamble, romance, smuggle, official, black_market.

不变量:

- 普通地盘最多一个控制帮派.
- 官府衙门作为特殊节点, 不进入普通占领结算.

## Event

事件是状态变化入口, 最小字段见 [../../simulation/event-schema.md](../../simulation/event-schema.md).

任何会影响世界状态、关系记忆或日志输出的重要行为都应记录事件.
