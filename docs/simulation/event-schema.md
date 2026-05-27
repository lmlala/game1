<!--
Project: game1
Created Date: 2026-05-27
Author: liming
Email: lmlala@aliyun.com
Copyright (c) 2025 FiuAI
-->

# 事件 Schema

**适用场景**: 需要实现事件类型、事件存储、回放或测试断言时.

## 基础结构

```json
{
  "schema_version": 1,
  "tick": 12,
  "event_id": "tick-0012-0004",
  "event_type": "MoneyStolen",
  "severity": "major",
  "actors": ["person:accountant_02"],
  "targets": ["gang:black_tiger"],
  "payload": {
    "amount": 80,
    "territory_id": "territory:gambling_house"
  },
  "caused_by_event_id": "tick-0012-0001"
}
```

## 字段规则

| 字段 | 必填 | 说明 |
|------|------|------|
| `schema_version` | 是 | 事件格式版本, V1 固定为 1 |
| `tick` | 是 | 发生 Tick, 从 1 开始递增 |
| `event_id` | 是 | 稳定唯一 ID, 同 seed 可复现 |
| `event_type` | 是 | 事件类型, 见 gameplay 事件目录 |
| `severity` | 是 | `critical`, `major`, `normal`, `minor` |
| `actors` | 是 | 主动实体 ID 列表 |
| `targets` | 是 | 目标实体 ID 列表 |
| `payload` | 是 | 结构化数据, 不放纯自然语言事实 |
| `caused_by_event_id` | 否 | 派生事件的原因事件 |

## 命名约定

- event_type 使用 PascalCase.
- 实体引用建议带类型前缀, 例如 `person:agou`, `gang:black_tiger`.
- payload 字段使用 snake_case.

## 兼容原则

- 新增字段必须有默认解释.
- 删除或改名字段需要提升 schema_version.
- 日志模板不能依赖未定义字段.
- 测试优先断言结构化字段, 不断言完整自然语言.
