extends Control

@onready var sim: DemoController = $DemoController
@onready var time_label: Label = $UI/HUD/TopBar/TimeLabel
@onready var metrics_label: Label = $UI/HUD/TopBar/MetricsLabel
@onready var event_log: RichTextLabel = $UI/HUD/RightPanel/VBox/EventLog
@onready var target_title: Label = $UI/HUD/RightPanel/VBox/TargetTitle
@onready var attr_list: VBoxContainer = $UI/HUD/RightPanel/VBox/AttrList
@onready var target_log: RichTextLabel = $UI/HUD/RightPanel/VBox/TargetLog
@onready var btn_back: Button = $UI/HUD/RightPanel/VBox/BtnBackEvents
@onready var world_view: Node2D = $WorldView
@onready var tick_timer: Timer = $TickTimer

var _panel_mode := "event"
var _selected_id := ""


func _ready() -> void:
	sim.tick_advanced.connect(_on_tick_advanced)
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
	world_view.setup(sim)
	world_view.entity_selected.connect(_on_entity_selected)
	_on_reset()


func _on_reset() -> void:
	sim.reset_demo(42)
	tick_timer.stop()
	$UI/HUD/TopBar/BtnPause.button_pressed = false
	sim.set_paused(false)
	_show_event_mode()
	_refresh_all()


func _on_pause_toggled(pressed: bool) -> void:
	sim.set_paused(pressed)
	if pressed:
		tick_timer.stop()
	else:
		tick_timer.start()
	_refresh_header()


func _on_next_day() -> void:
	sim.advance_tick()


func _on_timer_tick() -> void:
	if not sim.is_paused():
		sim.advance_tick()


func _on_tick_advanced(_tick: int) -> void:
	_refresh_all()


func _on_expand() -> void:
	var sel: Dictionary = sim.get_default_selection()
	var key: String = str(sel.get("territory", "territory:east_dock"))
	if _selected_id.begins_with("territory:"):
		key = _selected_id
	sim.queue_expand(key)


func _on_defend() -> void:
	var sel: Dictionary = sim.get_default_selection()
	var key: String = str(sel.get("territory", "territory:gambling_house"))
	if _selected_id.begins_with("territory:"):
		key = _selected_id
	sim.queue_defend(key)


func _on_reward() -> void:
	var sel: Dictionary = sim.get_default_selection()
	var key: String = str(sel.get("person", "person:accountant_black"))
	if _selected_id.begins_with("person:"):
		key = _selected_id
	sim.queue_reward(key, 80)


func _on_punish() -> void:
	var sel: Dictionary = sim.get_default_selection()
	var key: String = str(sel.get("person", "person:accountant_black_2"))
	if _selected_id.begins_with("person:"):
		key = _selected_id
	sim.queue_punish(key)


func _on_recruit() -> void:
	sim.queue_recruit(100)


func _on_entity_selected(entity_id: String) -> void:
	_selected_id = entity_id
	_show_target_mode(entity_id)


func _show_event_mode() -> void:
	_panel_mode = "event"
	event_log.visible = true
	target_title.visible = false
	attr_list.visible = false
	target_log.visible = false
	btn_back.visible = false
	_refresh_event_log()


func _show_target_mode(entity_id: String) -> void:
	_panel_mode = "target"
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
	time_label.text = sim.get_time_label()
	metrics_label.text = sim.get_metrics_summary()


func _refresh_event_log() -> void:
	event_log.clear()
	var logs: PackedStringArray = sim.get_recent_logs()
	for line in logs:
		event_log.append_text(line + "\n")


func _refresh_all() -> void:
	_refresh_header()
	if _panel_mode == "event":
		_refresh_event_log()
	elif _selected_id != "":
		_show_target_mode(_selected_id)
	world_view.refresh(sim)
