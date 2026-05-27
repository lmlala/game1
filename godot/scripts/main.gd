extends Control

## 模拟核心节点 (Rust DemoController 或 GDScript 占位).
var sim: Node

@onready var status_label: Label = $UI/HUD/TopBar/StatusLabel
@onready var time_label: Label = $UI/HUD/TopBar/TimeLabel
@onready var metrics_label: Label = $UI/HUD/TopBar/MetricsLabel
@onready var ui_log: RichTextLabel = $UI/HUD/RightPanel/VBox/UiLog
@onready var event_log: RichTextLabel = $UI/HUD/RightPanel/VBox/EventLog
@onready var target_title: Label = $UI/HUD/RightPanel/VBox/TargetTitle
@onready var attr_list: VBoxContainer = $UI/HUD/RightPanel/VBox/AttrList
@onready var target_log: RichTextLabel = $UI/HUD/RightPanel/VBox/TargetLog
@onready var btn_back: Button = $UI/HUD/RightPanel/VBox/BtnBackEvents
@onready var world_view: Node2D = $WorldView
@onready var tick_timer: Timer = $TickTimer

const FALLBACK_SCRIPT := preload("res://scripts/demo_sim_fallback.gd")

var _panel_mode := "event"
var _selected_id := ""
var _sim_mode := "unknown"


func _ready() -> void:
	_setup_ui_connections()
	_ui_log("Main 场景 _ready")
	call_deferred("_boot_sim")


func _setup_ui_connections() -> void:
	$UI/HUD/TopBar/BtnPause.toggled.connect(_on_pause_toggled)
	$UI/HUD/TopBar/BtnNext.pressed.connect(_on_next_day)
	$UI/HUD/TopBar/BtnReset.pressed.connect(_on_reset)
	$UI/HUD/ActionBar/BtnExpand.pressed.connect(_on_expand)
	$UI/HUD/ActionBar/BtnDefend.pressed.connect(_on_defend)
	$UI/HUD/ActionBar/BtnReward.pressed.connect(_on_reward)
	$UI/HUD/ActionBar/BtnPunish.pressed.connect(_on_punish)
	$UI/HUD/ActionBar/BtnRecruit.pressed.connect(_on_recruit)
	btn_back.pressed.connect(_show_event_mode)
	tick_timer.timeout.connect(_on_timer_tick)


func _boot_sim() -> void:
	sim = _ensure_sim()
	if sim == null:
		_sim_mode = "none"
		status_label.text = "Rust: 未连接"
		_ui_log("无法创建模拟核心.")
		return
	if sim.get_script() == FALLBACK_SCRIPT:
		_sim_mode = "fallback"
		status_label.text = "模式: GDScript 占位"
		_ui_log("警告: Rust 未加载, 使用占位模拟.")
	else:
		_sim_mode = "rust"
		status_label.text = "模式: Rust 核心"
		_ui_log("Rust DemoController 已连接.")
	if sim.has_signal("tick_advanced"):
		sim.tick_advanced.connect(_on_tick_advanced)
	world_view.setup(sim)
	world_view.entity_selected.connect(_on_entity_selected)
	world_view.z_index = 0
	$UI.z_index = 10
	_on_reset()


func _ensure_sim() -> Node:
	if has_node("DemoController"):
		return $DemoController
	if ClassDB.class_exists("DemoController"):
		var node: Node = ClassDB.instantiate("DemoController")
		node.name = "DemoController"
		add_child(node)
		_ui_log("ClassDB 实例化 DemoController 成功.")
		return node
	_ui_log("ClassDB 无 DemoController, 启用 GDScript 占位.")
	var fallback: Node = Node.new()
	fallback.name = "DemoController"
	fallback.set_script(FALLBACK_SCRIPT)
	add_child(fallback)
	return fallback


func _ui_log(message: String) -> void:
	var line := "[%s] %s" % [_time_stamp(), message]
	print(line)
	if ui_log:
		ui_log.append_text(line + "\n")
		ui_log.scroll_to_line(ui_log.get_line_count())


func _time_stamp() -> String:
	var t := Time.get_time_dict_from_system()
	return "%02d:%02d:%02d" % [t.hour, t.minute, t.second]


func _on_reset() -> void:
	_ui_log("点击: 重置")
	if sim == null:
		return
	sim.reset_demo(42)
	tick_timer.stop()
	$UI/HUD/TopBar/BtnPause.button_pressed = false
	sim.set_paused(false)
	_show_event_mode()
	_refresh_all()


func _on_pause_toggled(pressed: bool) -> void:
	_ui_log("点击: 暂停/继续 => %s" % ("暂停" if pressed else "继续"))
	if sim == null:
		return
	sim.set_paused(pressed)
	if pressed:
		tick_timer.stop()
	else:
		tick_timer.start()
	_refresh_header()


func _on_next_day() -> void:
	_ui_log("点击: 下一天")
	if sim == null:
		return
	var ok: bool = sim.advance_tick()
	_ui_log("advance_tick => %s" % ok)
	_refresh_all()


func _on_timer_tick() -> void:
	if sim and not sim.is_paused():
		_ui_log("定时器: 推进 1 tick")
		sim.advance_tick()
		_refresh_all()


func _on_tick_advanced(tick: int) -> void:
	_ui_log("信号 tick_advanced: %d" % tick)
	_refresh_all()


func _on_expand() -> void:
	_ui_log("点击: 扩张")
	if sim == null:
		return
	var sel: Dictionary = sim.get_default_selection()
	var key: String = str(sel.get("territory", "territory:east_dock"))
	if _selected_id.begins_with("territory:"):
		key = _selected_id
	var ok: bool = sim.queue_expand(key)
	_ui_log("queue_expand(%s) => %s" % [key, ok])


func _on_defend() -> void:
	_ui_log("点击: 防守")
	if sim == null:
		return
	var sel: Dictionary = sim.get_default_selection()
	var key: String = str(sel.get("territory", "territory:gambling_house"))
	if _selected_id.begins_with("territory:"):
		key = _selected_id
	var ok: bool = sim.queue_defend(key)
	_ui_log("queue_defend(%s) => %s" % [key, ok])


func _on_reward() -> void:
	_ui_log("点击: 奖赏")
	if sim == null:
		return
	var sel: Dictionary = sim.get_default_selection()
	var key: String = str(sel.get("person", "person:accountant_black"))
	if _selected_id.begins_with("person:"):
		key = _selected_id
	var ok: bool = sim.queue_reward(key, 80)
	_ui_log("queue_reward(%s) => %s" % [key, ok])


func _on_punish() -> void:
	_ui_log("点击: 处罚")
	if sim == null:
		return
	var sel: Dictionary = sim.get_default_selection()
	var key: String = str(sel.get("person", "person:accountant_black_2"))
	if _selected_id.begins_with("person:"):
		key = _selected_id
	var ok: bool = sim.queue_punish(key)
	_ui_log("queue_punish(%s) => %s" % [key, ok])


func _on_recruit() -> void:
	_ui_log("点击: 招募")
	if sim == null:
		return
	var ok: bool = sim.queue_recruit(100)
	_ui_log("queue_recruit => %s" % ok)


func _on_entity_selected(entity_id: String) -> void:
	_ui_log("选中: %s" % entity_id)
	_selected_id = entity_id
	_show_target_mode(entity_id)


func _show_event_mode() -> void:
	_panel_mode = "event"
	ui_log.visible = true
	event_log.visible = true
	target_title.visible = false
	attr_list.visible = false
	target_log.visible = false
	btn_back.visible = false
	_refresh_game_log()


func _show_target_mode(entity_id: String) -> void:
	if sim == null:
		return
	_panel_mode = "target"
	ui_log.visible = true
	event_log.visible = false
	target_title.visible = true
	attr_list.visible = true
	target_log.visible = true
	btn_back.visible = true
	var panel: Dictionary = sim.get_entity_panel(entity_id)
	target_title.text = str(panel.get("title", entity_id))
	for child in attr_list.get_children():
		child.queue_free()
	var attrs: Dictionary = panel.get("attributes", {})
	for key in attrs.keys():
		var row := Label.new()
		row.text = "%s: %s" % [key, attrs[key]]
		attr_list.add_child(row)
	var logs: PackedStringArray = panel.get("logs", PackedStringArray())
	target_log.clear()
	for line in logs:
		target_log.append_text(line + "\n")


func _refresh_header() -> void:
	if sim == null:
		return
	time_label.text = sim.get_time_label()
	metrics_label.text = sim.get_metrics_summary()


func _refresh_game_log() -> void:
	if sim == null:
		return
	event_log.clear()
	var logs: PackedStringArray = sim.get_recent_logs()
	for line in logs:
		event_log.append_text(line + "\n")
	event_log.scroll_to_line(event_log.get_line_count())


func _refresh_all() -> void:
	_refresh_header()
	if _panel_mode == "event":
		_refresh_game_log()
	elif _selected_id != "":
		_show_target_mode(_selected_id)
	world_view.refresh(sim)
