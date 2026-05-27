<!--
Project: game1
Created Date: 2026-05-27
Author: liming
Email: lmlala@aliyun.com
Copyright (c) 2025 FiuAI
-->

# 模拟验收指标

**适用场景**: 需要通过后端日志和指标判断 Demo V1 是否具备基础可玩性时.

## 固定验收

- 使用固定 seed 跑 30 Tick 和 100 Tick.
- 输出事件类型分布、帮派状态、地盘归属、角色存活、关系复杂度和日志样例.
- 同 seed 重跑时关键事件序列保持一致.

## 核心指标

| 指标 | 目标 |
|------|------|
| event_type_count | 100 Tick 内不少于 20 类主要事件 |
| funny_chain_count | 至少 5 条黑色幽默或反转链 |
| relationship_driven_count | 至少 3 条由关系/记忆驱动的关键事件链 |
| avg_events_per_tick | 不为 0, 且不造成日志过载 |
| gang_survival_count | 不应过早全部灭亡 |
| territory_change_count | 应出现多次地盘争夺或易主 |
| death_rate | 不能高到快速清空世界 |
| replay_stability | 同 seed 关键输出一致 |

## 需要关注的失衡信号

- 某帮派过早统一全部地盘.
- 所有角色短时间死亡或失效.
- 事件只有经济流水, 没有关系和戏剧变化.
- 日志只出现随机段子, 看不出规则因果.
- 投奔、复仇、误伤等事件完全不出现.

## 输出建议

每次验收输出三类内容:

1. `events.jsonl`: 结构化事件流.
2. `playable.log`: 玩家可读日志.
3. `metrics.json`: 指标快照和告警.

第一阶段以后端日志为准, UI 只在确认玩法有趣后再接入.
