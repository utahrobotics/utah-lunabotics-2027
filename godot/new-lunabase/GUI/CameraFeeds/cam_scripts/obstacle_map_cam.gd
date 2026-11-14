extends CameraStream

var is_streaming: bool = false
var port: String = "5000"
var connection_address: String
@onready var prefix := $ObstaclePrefix

func _ready() -> void:
	connection_address = get_parent().connection_prefix + port
	prefix.text = get_parent().connection_prefix

func _on_obstacle_map_shutoff_toggled(toggled_on: bool) -> void:
	is_streaming = toggled_on
	self.toggle_cam(connection_address,340,200)

func _on_obstacle_cam_text_field_text_changed(new_text: String) -> void:
		port = new_text
		connection_address = get_parent().connection_prefix+ ":" + port
	
