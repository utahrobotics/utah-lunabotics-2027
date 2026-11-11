extends CameraStream
var is_streaming: bool = false
var port: String = "5003"
var connection_address: String
@onready var prefix := $T265FrontPrefix
func _ready() -> void:
	connection_address = get_parent().connection_prefix + port
	prefix.text = get_parent().connection_prefix
	
func _on_t_265_front_toggle_toggled(toggled_on: bool) -> void:
	is_streaming = toggled_on
	self.toggle_cam(connection_address,340,200)


func _on_t_265_front_text_text_changed(new_text: String) -> void:
	port = new_text
	connection_address = get_parent().connection_prefix + port
