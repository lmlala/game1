extends Control

var sim: Node

@onready var status_label: Label = $UI/HUD/TopBar/StatusLabel
@onready var time_label: Label = $UI/HUD/TopBar/TimeLabel
@onready var metrics_label: Label = $UI/HUD/TopBar/MetricsLabel
@onready var btn_back: Button = $UI/HUD/RightPanel/VBox/BtnBackEvents
@onready var event_panel: VBoxContainer = $UI/HUD/RightPanel/VBox/EventPanel
@onready var ui_log: RichTextLabel = $UI/HUD/RightPanel/VBox/EventPanel/UiLog
@onready var event_log: RichTextLabel = $UI/HUD/RightPanel/VBox/EventPanel/EventLog
@onready var roster_panel: VBoxContainer = $UI/HUD/RightPanel/VBox/RosterPanel
@onready var gang_option: OptionButton = $UI/HUD/RightPanel/VBox/RosterPanel/GangOption
@onready var member_list: ItemList = $UI/HUD/RightPanel/VBox/RosterPanel/MemberList
@onready var person_panel: VBoxContainer = $UI/HUD/RightPanel/VBox/PersonPanel
@onready var person_title: Label = $UI/HUD/RightPanel/VBox/PersonPanel/PersonTitle
@onready var attr_list: VBoxContainer = $UI/HUD/RightPanel/VBox/PersonPanel/AttrPanel/AttrMargin/AttrList
@onready var person_log: RichTextLabel = $UI/HUD/RightPanel/VBox/PersonPanel/PersonLog
@onready var world_view: Node2D = $WorldView
@onready var tick_timer: Timer = $TickTimer
@onready var btn_gang_roster: Button = $UI/HUD/BottomBar/BtnGangRoster

const FALLBACK_SCRIPT: GDScript = preload("res://scripts/demo_sim_fallback.gd")
const SIM_CONFIG_PATH := "res://config/sim.cfg"

var _panel_mode := "event"
var _selected_person_id := ""
var _back_mode := "event"
var _sim_mode := "unknown"
var _gang_ids: Array[String] = []
var _tick_interval_sec := 1.0


func _ready() -> void:
	_load_sim_config()
	_setup_ui_connections()
	_ui_log("Main 场景 _ready, Tick 间隔 %.1fs" % _tick_interval_sec)
	call_deferred("_boot_sim")


func _load_sim_config() -> void:
	var cfg := ConfigFile.new()
	var err := cfg.load(SIM_CONFIG_PATH)
	if err != OK:
		push_warning("未找到 sim.cfg, 使用默认 Tick 间隔 1s")
		_tick_interval_sec = 1.0
	else:
		_tick_interval_sec = float(cfg.get_value("tick", "interval_sec", 1.0))
	tick_timer.wait_time = _tick_interval_sec


func _setup_ui_connections() -> void:
	$UI/HUD/BottomBar/BtnPause.toggled.connect(_on_pause_toggled)
	$UI/HUD/BottomBar/BtnReset.pressed.connect(_on_reset)
	btn_gang_roster.toggled.connect(_on_gang_roster_toggled)
	$UI/HUD/BottomBar/BtnExpand.pressed.connect(_on_expand)
	$UI/HUD/BottomBar/BtnDefend.pressed.connect(_on_defend)
	$UI/HUD/BottomBar/BtnReward.pressed.connect(_on_reward)
	$UI/HUD/BottomBar/BtnPunish.pressed.connect(_on_punish)
	$UI/HUD/BottomBar/BtnRecruit.pressed.connect(_on_recruit)
	btn_back.pressed.connect(_on_back_pressed)
	gang_option.item_selected.connect(_on_gang_selected)
	member_list.item_selected.connect(_on_member_selected)
	tick_timer.timeout.connect(_on_timer_tick)


func _boot_sim() -> void:
	sim = _ensure_sim()
	if sim == null:
		_sim_mode = "none"
		status_label.text = "Rust: 未连接"
		return
	if sim.get_script() == FALLBACK_SCRIPT:
		_sim_mode = "fallback"
		status_label.text = "模式: 占位 | 自动 %.1fs" % _tick_interval_sec
	else:
		_sim_mode = "rust"
		status_label.text = "模式: Rust | 自动 %.1fs" % _tick_interval_sec
	if sim.has_signal("tick_advanced"):
		sim.tick_advanced.connect(_on_tick_advanced)
	world_view.setup(sim)
	world_view.entity_selected.connect(_on_entity_selected)
	world_view.z_index = 0
	$UI.layer = 10
	_init_gang_options()
	_on_reset()


func _ensure_sim() -> Node:
	if has_node("DemoController"):
		return $DemoController
	if ClassDB.class_exists("DemoController"):
		var node: Node = ClassDB.instantiate("DemoController")
		node.name = "DemoController"
		add_child(node)
		return node
	var fallback: Node = Node.new()
	fallback.name = "DemoController"
	fallback.set_script(FALLBACK_SCRIPT)
	add_child(fallback)
	return fallback


func _start_auto_run() -> void:
	if sim == null:
		return
	sim.set_paused(false)
	$UI/HUD/BottomBar/BtnPause.button_pressed = false
	tick_timer.wait_time = _tick_interval_sec
	tick_timer.start()


func _stop_auto_run() -> void:
	tick_timer.stop()
	if sim:
		sim.set_paused(true)


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
	btn_gang_roster.button_pressed = false
	_panel_mode = "event"
	_back_mode = "event"
	_show_event_mode()
	_refresh_all()
	_start_auto_run()


func _on_pause_toggled(pressed: bool) -> void:
	_ui_log("点击: %s" % ("暂停" if pressed else "继续"))
	if sim == null:
		return
	if pressed:
		_stop_auto_run()
	else:
		_start_auto_run()
	_refresh_header()


func _on_timer_tick() -> void:
	if sim == null or sim.is_paused():
		return
	sim.advance_tick()


func _on_tick_advanced(tick: int) -> void:
	_refresh_all()


func _on_gang_roster_toggled(pressed: bool) -> void:
	if pressed:
		_ui_log("打开帮派名册")
		_panel_mode = "roster"
		_show_roster_mode()
	else:
		_ui_log("关闭帮派名册")
		_panel_mode = "event"
		_show_event_mode()


func _on_gang_selected(index: int) -> void:
	if index < 0 or index >= _gang_ids.size():
		return
	_refresh_member_list(_gang_ids[index])


func _on_member_selected(index: int, _click_at: Vector2, _mb: int) -> void:
	if index < 0:
		return
	var meta: Variant = member_list.get_item_metadata(index)
	if meta == null:
		return
	var person_id := str(meta)
	_ui_log("名册选中: %s" % person_id)
	_selected_person_id = person_id
	_back_mode = "roster"
	_show_person_mode(person_id)


func _on_back_pressed() -> void:
	if _panel_mode == "person":
		if _back_mode == "roster":
			_panel_mode = "roster"
			_show_roster_mode()
		else:
			_panel_mode = "event"
			btn_gang_roster.button_pressed = false
			_show_event_mode()
	elif _panel_mode == "roster":
		_panel_mode = "event"
		btn_gang_roster.button_pressed = false
		_show_event_mode()


func _on_entity_selected(entity_id: String) -> void:
	if not entity_id.begins_with("person:"):
		return
	_selected_person_id = entity_id
	_back_mode = "event"
	_show_person_mode(entity_id)


func _show_event_mode() -> void:
	_panel_mode = "event"
	btn_back.visible = false
	event_panel.visible = true
	roster_panel.visible = false
	person_panel.visible = false
	_refresh_game_log()


func _show_roster_mode() -> void:
	_panel_mode = "roster"
	btn_back.visible = true
	btn_back.text = "返回江湖事件"
	event_panel.visible = false
	roster_panel.visible = true
	person_panel.visible = false
	_init_gang_options()
	if gang_option.item_count > 0:
		var idx := gang_option.selected
		if idx < 0:
			idx = 0
			gang_option.select(0)
		_on_gang_selected(idx)


func _show_person_mode(person_id: String) -> void:
	_panel_mode = "person"
	_selected_person_id = person_id
	btn_back.visible = true
	btn_back.text = "返回" + ("名册" if _back_mode == "roster" else "江湖事件")
	event_panel.visible = false
	roster_panel.visible = false
	person_panel.visible = true
	_fill_person_panel(person_id)


func _fill_person_panel(person_id: String) -> void:
	if sim == null:
		return
	var panel: Dictionary = sim.get_entity_panel(person_id)
	person_title.text = str(panel.get("title", person_id))
	for child in attr_list.get_children():
		child.queue_free()
	var attrs: Dictionary = panel.get("attributes", {})
	for key in attrs.keys():
		var row := Label.new()
		row.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
		row.text = "%s: %s" % [key, attrs[key]]
		attr_list.add_child(row)
	person_log.clear()
	var logs: PackedStringArray = panel.get("logs", PackedStringArray())
	for i in range(logs.size()):
		person_log.append_text(str(logs[i]) + "\n")
	if logs.is_empty():
		person_log.append_text("暂无个人相关日志。\n")
	person_log.scroll_to_line(0)


func _init_gang_options() -> void:
	gang_option.clear()
	member_list.clear()
	_gang_ids.clear()
	if sim == null:
		return
	var meta: Dictionary = _call_roster_meta("get_gang_roster_meta")
	var ids: PackedStringArray = meta.get("ids", PackedStringArray())
	var labels: PackedStringArray = meta.get("labels", PackedStringArray())
	if ids.is_empty():
		_ui_log("警告: 帮派列表为空, 请确认 Rust 已 build 并重启 Godot")
		return
	for i in range(ids.size()):
		_gang_ids.append(str(ids[i]))
		var label := str(labels[i]) if i < labels.size() else str(ids[i])
		gang_option.add_item(label)
	if _gang_ids.size() > 0:
		gang_option.select(0)
		_refresh_member_list(_gang_ids[0])


func _call_roster_meta(method: String, arg: String = "") -> Dictionary:
	if sim == null:
		return {}
	var result: Variant
	if arg.is_empty():
		result = sim.call(method)
	else:
		result = sim.call(method, arg)
	if result is Dictionary:
		return result
	if typeof(result) == TYPE_DICTIONARY:
		return result
	_ui_log("警告: %s 返回类型异常: %s" % [method, typeof(result)])
	return {}


func _refresh_member_list(gang_id: String) -> void:
	member_list.clear()
	if sim == null or gang_id.is_empty():
		return
	var meta: Dictionary = _call_roster_meta("get_member_roster_for_gang", gang_id)
	var ids: PackedStringArray = meta.get("ids", PackedStringArray())
	var labels: PackedStringArray = meta.get("labels", PackedStringArray())
	for i in range(ids.size()):
		var pid := str(ids[i])
		var label := str(labels[i]) if i < labels.size() else pid
		member_list.add_item(label)
		member_list.set_item_metadata(i, pid)


func _on_expand() -> void:
	_ui_log("点击: 扩张")
	if sim == null:
		return
	var sel: Dictionary = sim.get_default_selection()
	var key: String = str(sel.get("territory", "territory:east_dock"))
	if _selected_person_id.begins_with("territory:"):
		key = _selected_person_id
	sim.queue_expand(key)


func _on_defend() -> void:
	_ui_log("点击: 防守")
	if sim == null:
		return
	var sel: Dictionary = sim.get_default_selection()
	var key: String = str(sel.get("territory", "territory:gambling_house"))
	sim.queue_defend(key)


func _on_reward() -> void:
	_ui_log("点击: 奖赏")
	if sim == null:
		return
	var sel: Dictionary = sim.get_default_selection()
	var key: String = str(sel.get("person", "person:accountant_black"))
	if _selected_person_id.begins_with("person:"):
		key = _selected_person_id
	sim.queue_reward(key, 80)


func _on_punish() -> void:
	_ui_log("点击: 处罚")
	if sim == null:
		return
	var sel: Dictionary = sim.get_default_selection()
	var key: String = str(sel.get("person", "person:accountant_black_2"))
	if _selected_person_id.begins_with("person:"):
		key = _selected_person_id
	sim.queue_punish(key)


func _on_recruit() -> void:
	_ui_log("点击: 招募")
	if sim:
		sim.queue_recruit(100)


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
	match _panel_mode:
		"event":
			_refresh_game_log()
		"roster":
			if gang_option.selected >= 0 and gang_option.selected < _gang_ids.size():
				_refresh_member_list(_gang_ids[gang_option.selected])
		"person":
			if _selected_person_id != "":
				_fill_person_panel(_selected_person_id)
	world_view.refresh(sim)
