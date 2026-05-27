extends Node

signal tick_advanced(tick: int)

## GDScript 占位模拟: 仅在 Rust GDExtension 未加载时使用, 便于验证 UI 与按钮日志.
var _tick: int = 0
var _paused: bool = false
var _logs: PackedStringArray = PackedStringArray()


func _ready() -> void:
	reset_demo(42)


func reset_demo(_seed: int) -> void:
	_tick = 0
	_paused = false
	_logs = PackedStringArray([
		"【占位】Rust 扩展未加载, 当前为 GDScript 演示数据.",
		"【占位】请在项目根执行 ./scripts/build-rust.sh 后重启 Godot.",
	])
	emit_signal("tick_advanced", _tick)


func advance_tick() -> bool:
	if _paused:
		return false
	_tick += 1
	_logs.append("【第%d天】【占位】江湖自行推进了一天." % _tick)
	if _logs.size() > 40:
		_logs = _logs.slice(_logs.size() - 40, _logs.size())
	emit_signal("tick_advanced", _tick)
	return true


func set_paused(paused: bool) -> void:
	_paused = paused


func is_paused() -> bool:
	return _paused


func get_time_label() -> String:
	var state := "暂停" if _paused else "运行(占位)"
	return "第 %d 天 | %s" % [_tick, state]


func get_metrics_summary() -> String:
	return "占位模式 | 事件 %d 条" % _logs.size()


func get_recent_logs() -> PackedStringArray:
	return _logs


func get_entities(kind: String) -> Array:
	var out: Array = []
	if kind == "person":
		out.append({"id": "person:demo", "label": "占位侠客", "x": 300.0, "y": 200.0})
	elif kind == "territory":
		out.append({"id": "territory:demo", "label": "占位地盘", "x": 500.0, "y": 250.0})
	elif kind == "item":
		out.append({"id": "item:demo", "label": "占位物品", "x": 400.0, "y": 300.0})
	return out


func get_entity_panel(entity_id: String) -> Dictionary:
	return {
		"title": entity_id,
		"kind": "fallback",
		"attributes": {"模式": "GDScript 占位"},
		"logs": _logs,
	}


func get_default_selection() -> Dictionary:
	return {
		"person": "person:demo",
		"territory": "territory:demo",
		"item": "item:demo",
	}


func queue_expand(_key: String) -> bool:
	_logs.append("【占位】帮主下令: 扩张")
	return true


func queue_defend(_key: String) -> bool:
	_logs.append("【占位】帮主下令: 防守")
	return true


func queue_reward(_key: String, money: int) -> bool:
	_logs.append("【占位】帮主下令: 奖赏 %d 两" % money)
	return true


func queue_punish(_key: String) -> bool:
	_logs.append("【占位】帮主下令: 处罚 %s" % _key)
	return true


func queue_recruit(_budget: int) -> bool:
	_logs.append("【占位】帮主下令: 招募 预算 %d" % _budget)
	return true


func get_gangs() -> Array:
	return [
		{"id": "gang:black_tiger", "name": "黑虎帮", "member_count": 2, "defeated": false},
		{"id": "gang:axe_gang", "name": "斧头帮", "member_count": 1, "defeated": false},
		{"id": "gang:green_dragon", "name": "青龙会", "member_count": 1, "defeated": false},
	]


func get_persons_by_gang(gang_key: String) -> Array:
	if gang_key == "gang:black_tiger":
		return [
			{
				"id": "person:demo",
				"label": "占位侠客 [杂役]",
				"name": "占位侠客",
				"role": "杂役",
				"alive": true,
			},
			{
				"id": "person:accountant_black",
				"label": "铁算盘 [账房]",
				"name": "铁算盘",
				"role": "账房",
				"alive": true,
			},
		]
	return [
		{
			"id": "person:demo",
			"label": "路人 [杂役]",
			"name": "路人",
			"role": "杂役",
			"alive": true,
		},
	]
