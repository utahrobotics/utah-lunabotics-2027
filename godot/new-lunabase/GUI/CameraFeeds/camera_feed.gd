extends Node2D

@onready var camstream := $CameraStream
@onready var button := $Button
@onready var camtext := $TextureRect

var isCamOn := false

func _on_button_pressed() -> void:
	print("BUTTON PRESSED")
	camstream.connect_camera("127.0.0.1:4002")
	isCamOn = true

func _on_button_2_pressed() -> void:
	print("STOP")
	camstream.shutoff_cam()
	isCamOn = false

func _process(delta: float) -> void:
	if isCamOn:
		camstream.get_texture(camtext)
