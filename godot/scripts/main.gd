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
@onready var attr_list: VBoxContainer = $UI/HUD/RightPanel/VBox/PersonPanel/AttrPanel/AttrScroll/AttrMargin/AttrList
@onready var person_log: RichTextLabel = $UI/HUD/RightPanel/VBox/PersonPanel/PersonLog
@onready var world_view: Node2D = $WorldView
@onready var tick_timer: Timer = $TickTimer
@onready var btn_gang_roster: Button = $UI/HUD/BottomBar/BtnGangRoster
@onready var btn_event_history: Button = $UI/HUD/BottomBar/BtnEventHistory
@onready var event_history_panel: VBoxContainer = $UI/HUD/RightPanel/VBox/EventHistoryPanel
@onready var filter_gang: OptionButton = $UI/HUD/RightPanel/VBox/EventHistoryPanel/FilterGang
@onready var filter_type: OptionButton = $UI/HUD/RightPanel/VBox/EventHistoryPanel/FilterType
@onready var filter_severity: OptionButton = $UI/HUD/RightPanel/VBox/EventHistoryPanel/FilterSeverity
@onready var filter_entity: OptionButton = $UI/HUD/RightPanel/VBox/EventHistoryPanel/FilterEntity
@onready var event_history_list: ItemList = $UI/HUD/RightPanel/VBox/EventHistoryPanel/EventHistoryList
@onready var event_detail_panel: VBoxContainer = $UI/HUD/RightPanel/VBox/EventDetailPanel
@onready var event_detail_title: Label = $UI/HUD/RightPanel/VBox/EventDetailPanel/EventDetailTitle
@onready var event_attr_list: VBoxContainer = $UI/HUD/RightPanel/VBox/EventDetailPanel/EventAttrPanel/EventAttrScroll/EventAttrMargin/EventAttrList
@onready var event_detail_log: RichTextLabel = $UI/HUD/RightPanel/VBox/EventDetailPanel/EventDetailLog
@onready var related_events_list: ItemList = $UI/HUD/RightPanel/VBox/EventDetailPanel/RelatedEventsList
@onready var participants_list: ItemList = $UI/HUD/RightPanel/VBox/EventDetailPanel/ParticipantsList

const FALLBACK_SCRIPT: GDScript = preload("res://scripts/demo_sim_fallback.gd")
const SIM_CONFIG_PATH := "res://config/sim.cfg"

const DEFAULT_GANG_ROSTER = [
	{"id": "gang:black_tiger", "name": "黑虎帮"},
	{"id": "gang:axe_gang", "name": "斧头帮"},
	{"id": "gang:green_dragon", "name": "青龙会"},
]


var _panel_mode := "event"
var _selected_person_id := ""
var _selected_event_id := ""
var _back_mode := "event"
var _event_filter_ids: Dictionary = {
	"gang": PackedStringArray(),
	"type": PackedStringArray(),
	"severity": PackedStringArray(),
	"entity": PackedStringArray(),
}
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



func _has_event_history_btn() -> bool:
	return btn_event_history != null and is_instance_valid(btn_event_history)


func _set_gang_roster_pressed(pressed: bool) -> void:
	if btn_gang_roster != null and is_instance_valid(btn_gang_roster):
		btn_gang_roster.button_pressed = pressed


func _set_event_history_pressed(pressed: bool) -> void:
	if _has_event_history_btn():
		btn_event_history.button_pressed = pressed

func _setup_ui_connections() -> void:
	$UI/HUD/BottomBar/BtnPause.toggled.connect(_on_pause_toggled)
	$UI/HUD/BottomBar/BtnReset.pressed.connect(_on_reset)
	if btn_gang_roster != null:
		btn_gang_roster.toggled.connect(_on_gang_roster_toggled)
	if _has_event_history_btn():
		btn_event_history.toggled.connect(_on_event_history_toggled)
	filter_gang.item_selected.connect(_on_event_filter_changed)
	filter_type.item_selected.connect(_on_event_filter_changed)
	filter_severity.item_selected.connect(_on_event_filter_changed)
	filter_entity.item_selected.connect(_on_event_filter_changed)
	event_history_list.item_selected.connect(_on_event_history_selected)
	event_history_list.item_clicked.connect(_on_event_history_clicked)
	related_events_list.item_selected.connect(_on_related_event_selected)
	related_events_list.item_clicked.connect(_on_related_event_clicked)
	participants_list.item_selected.connect(_on_participant_selected)
	participants_list.item_clicked.connect(_on_participant_clicked)
	$UI/HUD/BottomBar/BtnExpand.pressed.connect(_on_expand)
	$UI/HUD/BottomBar/BtnDefend.pressed.connect(_on_defend)
	$UI/HUD/BottomBar/BtnReward.pressed.connect(_on_reward)
	$UI/HUD/BottomBar/BtnPunish.pressed.connect(_on_punish)
	$UI/HUD/BottomBar/BtnRecruit.pressed.connect(_on_recruit)
	btn_back.pressed.connect(_on_back_pressed)
	gang_option.item_selected.connect(_on_gang_selected)
	member_list.item_selected.connect(_on_member_selected)
	member_list.item_clicked.connect(_on_member_clicked)
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
	_set_gang_roster_pressed(false)
	_set_event_history_pressed(false)
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
		_set_event_history_pressed(false)
		_ui_log("打开帮派名册")
		_panel_mode = "roster"
		_show_roster_mode()
	elif _panel_mode == "person" and _back_mode == "roster":
		_ui_log("关闭帮派名册 (保留人物面板)")
		# 名册按钮仅收起列表，人物详情仍显示
		pass
	else:
		_ui_log("关闭帮派名册")
		_panel_mode = "event"
		_show_event_mode()


func _on_gang_selected(index: int) -> void:
	if index < 0 or index >= _gang_ids.size():
		return
	_refresh_member_list(_gang_ids[index])


func _on_member_selected(index: int) -> void:
	_open_person_from_member_index(index)


func _on_member_clicked(index: int, _at: Vector2, _mb: int) -> void:
	_open_person_from_member_index(index)


func _open_person_from_member_index(index: int) -> void:
	if index < 0 or index >= member_list.item_count:
		return
	var meta: Variant = member_list.get_item_metadata(index)
	if meta == null or str(meta).is_empty():
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
		elif _back_mode == "event_detail":
			_show_event_detail_mode(_selected_event_id)
		else:
			_panel_mode = "event"
			_set_gang_roster_pressed(false)
			_set_event_history_pressed(false)
			_show_event_mode()
	elif _panel_mode == "event_detail":
		_show_event_history_mode()
	elif _panel_mode == "event_history":
		_panel_mode = "event"
		_set_event_history_pressed(false)
		_show_event_mode()
	elif _panel_mode == "roster":
		_panel_mode = "event"
		_set_gang_roster_pressed(false)
		_show_event_mode()


func _on_entity_selected(entity_id: String) -> void:
	if not entity_id.begins_with("person:"):
		return
	_selected_person_id = entity_id
	_back_mode = "event"
	_show_person_mode(entity_id)



func _on_event_history_toggled(pressed: bool) -> void:
	if pressed:
		_set_gang_roster_pressed(false)
		_ui_log("打开事件历史")
		_back_mode = "event"
		_show_event_history_mode()
	elif _panel_mode == "event_detail" and _back_mode == "event_history":
		_ui_log("关闭事件历史 (保留事件详情)")
		pass
	elif _panel_mode in ["event_history", "event_detail"]:
		_ui_log("关闭事件历史")
		_panel_mode = "event"
		_show_event_mode()
	else:
		_panel_mode = "event"
		_show_event_mode()


func _on_event_filter_changed(_index: int) -> void:
	if _panel_mode == "event_history":
		_refresh_event_history_list()


func _on_event_history_selected(index: int) -> void:
	_open_event_detail_from_history_index(index)


func _on_event_history_clicked(index: int, _at: Vector2, _mb: int) -> void:
	_open_event_detail_from_history_index(index)


func _open_event_detail_from_history_index(index: int) -> void:
	if index < 0 or index >= event_history_list.item_count:
		return
	var meta: Variant = event_history_list.get_item_metadata(index)
	if meta == null or str(meta).is_empty():
		return
	_show_event_detail_mode(str(meta))


func _on_related_event_selected(index: int) -> void:
	_open_related_event_index(index)


func _on_related_event_clicked(index: int, _at: Vector2, _mb: int) -> void:
	_open_related_event_index(index)


func _open_related_event_index(index: int) -> void:
	if index < 0:
		return
	var meta: Variant = related_events_list.get_item_metadata(index)
	if meta == null or str(meta).is_empty():
		return
	_show_event_detail_mode(str(meta))


func _on_participant_selected(index: int) -> void:
	_open_participant_index(index)


func _on_participant_clicked(index: int, _at: Vector2, _mb: int) -> void:
	_open_participant_index(index)


func _open_participant_index(index: int) -> void:
	if index < 0:
		return
	var meta: Variant = participants_list.get_item_metadata(index)
	if meta == null:
		return
	var entity_id := str(meta)
	if entity_id.begins_with("person:"):
		_selected_person_id = entity_id
		_back_mode = "event_detail"
		_show_person_mode(entity_id)


func _hide_side_panels() -> void:
	event_panel.visible = false
	roster_panel.visible = false
	person_panel.visible = false
	event_history_panel.visible = false
	event_detail_panel.visible = false


func _show_event_history_mode() -> void:
	_panel_mode = "event_history"
	btn_back.visible = true
	btn_back.text = "返回江湖事件"
	_hide_side_panels()
	event_history_panel.visible = true
	_init_event_history_filters()
	_refresh_event_history_list()


func _show_event_detail_mode(event_id: String) -> void:
	_panel_mode = "event_detail"
	_selected_event_id = event_id
	_back_mode = "event_history"
	btn_back.visible = true
	btn_back.text = "返回事件列表"
	_hide_side_panels()
	event_detail_panel.visible = true
	_fill_event_detail_panel(event_id)


func _init_event_history_filters() -> void:
	if sim == null:
		return
	var meta: Dictionary = _fetch_event_history_filter_meta()
	_fill_filter_option(filter_gang, "gang", meta)
	_fill_filter_option(filter_type, "type", meta)
	_fill_filter_option(filter_severity, "severity", meta)
	_fill_filter_option(filter_entity, "entity", meta)


func _fetch_event_history_filter_meta() -> Dictionary:
	if sim == null:
		return {}
	if sim.get_script() == FALLBACK_SCRIPT:
		if sim.has_method("get_event_history_filter_meta"):
			return sim.get_event_history_filter_meta()
		return {}
	if _safe_has_method("get_event_history_filter_meta"):
		var r: Variant = sim.call("get_event_history_filter_meta")
		if r is Dictionary:
			return r
	return {}


func _fill_filter_option(option: OptionButton, key: String, meta: Dictionary) -> void:
	option.clear()
	var ids_key := "%s_ids" % key
	var labels_key := "%s_labels" % key
	var ids: PackedStringArray = meta.get(ids_key, PackedStringArray())
	var labels: PackedStringArray = meta.get(labels_key, PackedStringArray())
	if ids.is_empty():
		option.add_item("全部")
		option.set_item_metadata(0, "*")
		_event_filter_ids[key] = PackedStringArray(["*"])
		return
	_event_filter_ids[key] = ids
	for i in range(ids.size()):
		var label := str(labels[i]) if i < labels.size() else str(ids[i])
		option.add_item(label)
		option.set_item_metadata(i, str(ids[i]))
	option.select(0)


func _current_filter_id(key: String, option: OptionButton) -> String:
	var idx := option.selected
	if idx < 0:
		return "*"
	var meta: Variant = option.get_item_metadata(idx)
	if meta == null:
		return "*"
	return str(meta)


func _refresh_event_history_list() -> void:
	event_history_list.clear()
	if sim == null:
		return
	var gang_id := _current_filter_id("gang", filter_gang)
	var type_id := _current_filter_id("type", filter_type)
	var sev_id := _current_filter_id("severity", filter_severity)
	var entity_id := _current_filter_id("entity", filter_entity)
	var rows: Array = []
	if sim.get_script() == FALLBACK_SCRIPT and sim.has_method("query_event_history"):
		rows = sim.query_event_history(gang_id, type_id, sev_id, entity_id, 80)
	elif _safe_has_method("query_event_history"):
		var r: Variant = sim.call(
			"query_event_history", gang_id, type_id, sev_id, entity_id, 80
		)
		if r is Array:
			rows = r
	for row in rows:
		var d: Dictionary = row if row is Dictionary else {}
		var eid := str(d.get("id", ""))
		if eid.is_empty():
			continue
		var tick := int(d.get("tick", 0))
		var type_label := str(d.get("type_label", ""))
		var summary := str(d.get("summary", eid))
		var line := "第%d天 [%s] %s" % [tick, type_label, summary]
		if line.length() > 120:
			line = line.substr(0, 117) + "..."
		event_history_list.add_item(line)
		event_history_list.set_item_metadata(event_history_list.item_count - 1, eid)


func _fill_event_detail_panel(event_id: String) -> void:
	if sim == null:
		return
	var panel: Dictionary = {}
	if sim.get_script() == FALLBACK_SCRIPT and sim.has_method("get_event_detail"):
		panel = sim.get_event_detail(event_id)
	elif _safe_has_method("get_event_detail"):
		var r: Variant = sim.call("get_event_detail", event_id)
		if r is Dictionary:
			panel = r
	if not bool(panel.get("found", true)) and panel.is_empty():
		event_detail_title.text = "未找到事件"
		return
	event_detail_title.text = str(panel.get("title", event_id))
	for child in event_attr_list.get_children():
		child.queue_free()
	var attrs: Dictionary = panel.get("attributes", {})
	for key in attrs.keys():
		var row := Label.new()
		row.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
		row.text = "%s: %s" % [key, attrs[key]]
		event_attr_list.add_child(row)
	event_detail_log.clear()
	event_detail_log.append_text(str(panel.get("log", "")) + "\n")
	event_detail_log.scroll_to_line(0)
	related_events_list.clear()
	var related: Array = panel.get("related_events", [])
	for item in related:
		var d: Dictionary = item if item is Dictionary else {}
		var rid := str(d.get("id", ""))
		if rid.is_empty():
			continue
		var line := "第%d天 [%s] %s" % [
			int(d.get("tick", 0)),
			str(d.get("type_label", "")),
			str(d.get("summary", rid)),
		]
		if line.length() > 110:
			line = line.substr(0, 107) + "..."
		related_events_list.add_item(line)
		related_events_list.set_item_metadata(related_events_list.item_count - 1, rid)
	participants_list.clear()
	var parts: Array = panel.get("participants", [])
	for item in parts:
		var d: Dictionary = item if item is Dictionary else {}
		var pid := str(d.get("id", ""))
		if pid.is_empty():
			continue
		participants_list.add_item(str(d.get("label", pid)))
		participants_list.set_item_metadata(participants_list.item_count - 1, pid)


func _show_event_mode() -> void:
	_panel_mode = "event"
	btn_back.visible = false
	_hide_side_panels()
	event_panel.visible = true
	_refresh_game_log()


func _show_roster_mode() -> void:
	_panel_mode = "roster"
	btn_back.visible = true
	btn_back.text = "返回江湖事件"
	_hide_side_panels()
	roster_panel.visible = true
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
	if _back_mode == "roster":
		btn_back.text = "返回名册"
	elif _back_mode == "event_detail":
		btn_back.text = "返回事件详情"
	else:
		btn_back.text = "返回江湖事件"
	_hide_side_panels()
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
	# 倒序：最新在顶部
	person_log.scroll_to_line(0)


func _init_gang_options() -> void:
	gang_option.clear()
	member_list.clear()
	_gang_ids.clear()
	if sim == null:
		return
	var meta: Dictionary = _fetch_gang_roster_meta()
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


func _safe_has_method(method: String) -> bool:
	if sim == null:
		return false
	if sim.has_method(method):
		return true
	# GDScript 占位脚本
	if sim.get_script() == FALLBACK_SCRIPT:
		return true
	return false


func _fetch_gang_roster_meta() -> Dictionary:
	if sim == null:
		return {}
	# 1) GDScript 占位
	if sim.get_script() == FALLBACK_SCRIPT:
		return sim.get_gang_roster_meta()
	# 2) Rust 新 API (需重新 build)
	if _safe_has_method("get_gang_roster_meta"):
		var r: Variant = sim.call("get_gang_roster_meta")
		if r is Dictionary:
			return r
	# 3) Rust 旧 API get_gangs
	if _safe_has_method("get_gangs"):
		var built := _gang_meta_from_get_gangs()
		if not built.get("ids", PackedStringArray()).is_empty():
			return built
	# 4) 从 get_entities("person") 统计
	var from_entities := _gang_meta_from_entities()
	if not from_entities.get("ids", PackedStringArray()).is_empty():
		return from_entities
	# 5) 写死默认三帮
	return _gang_meta_from_defaults()


func _gang_meta_from_get_gangs() -> Dictionary:
	var ids := PackedStringArray()
	var labels := PackedStringArray()
	var gangs: Variant = sim.call("get_gangs")
	if gangs == null or not (gangs is Array):
		return {"ids": ids, "labels": labels}
	for g in gangs:
		var d: Dictionary = g if g is Dictionary else {}
		var gid := str(d.get("id", ""))
		if gid.is_empty():
			continue
		var name := str(d.get("name", gid))
		var count := int(d.get("member_count", 0))
		ids.append(gid)
		labels.append("%s (%d人)" % [name, count])
	return {"ids": ids, "labels": labels}


func _gang_meta_from_entities() -> Dictionary:
	var ids := PackedStringArray()
	var labels := PackedStringArray()
	if not _safe_has_method("get_entities"):
		return {"ids": ids, "labels": labels}
	var persons: Variant = sim.call("get_entities", "person")
	if persons == null or not (persons is Array):
		return {"ids": ids, "labels": labels}
	var counts: Dictionary = {}
	for g in DEFAULT_GANG_ROSTER:
		counts[str(g["id"])] = 0
	for p in persons:
		var d: Dictionary = p if p is Dictionary else {}
		var gid := str(d.get("gang_id", ""))
		if counts.has(gid):
			counts[gid] = int(counts[gid]) + 1
	for g in DEFAULT_GANG_ROSTER:
		var gid := str(g["id"])
		var name := str(g["name"])
		var n := int(counts.get(gid, 0))
		ids.append(gid)
		labels.append("%s (%d人)" % [name, n])
	return {"ids": ids, "labels": labels}


func _gang_meta_from_defaults() -> Dictionary:
	var ids := PackedStringArray()
	var labels := PackedStringArray()
	for g in DEFAULT_GANG_ROSTER:
		ids.append(str(g["id"]))
		labels.append(str(g["name"]))
	return {"ids": ids, "labels": labels}


func _fetch_member_roster_meta(gang_id: String) -> Dictionary:
	if sim == null or gang_id.is_empty():
		return {}
	if sim.get_script() == FALLBACK_SCRIPT:
		return sim.get_member_roster_for_gang(gang_id)
	if _safe_has_method("get_member_roster_for_gang"):
		var r: Variant = sim.call("get_member_roster_for_gang", gang_id)
		if r is Dictionary:
			return r
	if _safe_has_method("get_persons_by_gang"):
		var built := _member_meta_from_get_persons(gang_id)
		if not built.get("ids", PackedStringArray()).is_empty():
			return built
	return _member_meta_from_entities(gang_id)


func _member_meta_from_get_persons(gang_id: String) -> Dictionary:
	var ids := PackedStringArray()
	var labels := PackedStringArray()
	var members: Variant = sim.call("get_persons_by_gang", gang_id)
	if members == null or not (members is Array):
		return {"ids": ids, "labels": labels}
	for m in members:
		var d: Dictionary = m if m is Dictionary else {}
		ids.append(str(d.get("id", "")))
		labels.append(str(d.get("label", str(d.get("id", "")))))
	return {"ids": ids, "labels": labels}


func _member_meta_from_entities(gang_id: String) -> Dictionary:
	var ids := PackedStringArray()
	var labels := PackedStringArray()
	if not _safe_has_method("get_entities"):
		return {"ids": ids, "labels": labels}
	var persons: Variant = sim.call("get_entities", "person")
	if persons == null or not (persons is Array):
		return {"ids": ids, "labels": labels}
	for p in persons:
		var d: Dictionary = p if p is Dictionary else {}
		if str(d.get("gang_id", "")) != gang_id:
			continue
		ids.append(str(d.get("id", "")))
		labels.append(str(d.get("label", str(d.get("id", "")))))
	return {"ids": ids, "labels": labels}


func _refresh_member_list(gang_id: String) -> void:
	member_list.clear()
	if sim == null or gang_id.is_empty():
		return
	var meta: Dictionary = _fetch_member_roster_meta(gang_id)
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
		"event_history":
			_refresh_event_history_list()
		"event_detail":
			if _selected_event_id != "":
				_fill_event_detail_panel(_selected_event_id)
	world_view.refresh(sim)
