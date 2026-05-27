extends Node2D

signal entity_selected(entity_id: String)

const TOKEN_SCENE := preload("res://scenes/world/entity_token.tscn")

var _sim: DemoController


func setup(controller: DemoController) -> void:
	_sim = controller


func refresh(controller: DemoController) -> void:
	_sim = controller
	for child in get_children():
		child.queue_free()
	if _sim == null:
		return
	_spawn_group("territory", Color(0.2, 0.5, 0.8, 0.85), 72, 36)
	_spawn_group("person", Color(0.85, 0.35, 0.25, 0.9), 28, 28)
	_spawn_group("item", Color(0.75, 0.7, 0.2, 0.9), 18, 18)


func _spawn_group(kind: String, color: Color, w: float, h: float) -> void:
	var entities: Array = _sim.get_entities(kind)
	for data in entities:
		var token: Area2D = TOKEN_SCENE.instantiate()
		token.position = Vector2(float(data.get("x", 0)), float(data.get("y", 0)))
		token.setup(str(data.get("id", "")), str(data.get("label", "")), color, Vector2(w, h))
		token.clicked.connect(_on_token_clicked)
		add_child(token)


func _on_token_clicked(entity_id: String) -> void:
	entity_selected.emit(entity_id)
