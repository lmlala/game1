extends Area2D

signal clicked(entity_id: String)

var _entity_id := ""


func setup(entity_id: String, label_text: String, color: Color, size: Vector2) -> void:
	_entity_id = entity_id
	var rect := $ColorRect
	rect.size = size
	rect.position = -size * 0.5
	rect.color = color
	$Label.text = label_text
	$Label.position = Vector2(-size.x * 0.5, size.y * 0.5 + 2)
	$Label.custom_minimum_size = Vector2(size.x + 40, 20)
	$CollisionShape2D.shape = RectangleShape2D.new()
	$CollisionShape2D.shape.size = size


func _input_event(_viewport: Node, event: InputEvent, _shape_idx: int) -> void:
	if event is InputEventMouseButton and event.pressed and event.button_index == MOUSE_BUTTON_LEFT:
		clicked.emit(_entity_id)
