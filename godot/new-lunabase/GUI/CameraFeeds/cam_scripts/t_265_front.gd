extends CameraStream

var is_streaming: bool = false
var connection_address:= "192.168.0.100:5003"


func _on_t_265_front_toggle_toggled(toggled_on: bool) -> void:
	is_streaming = toggled_on
	self.toggle_cam(connection_address,340,200)


func _on_t_265_front_text_text_changed(new_text: String) -> void:
	connection_address = new_text
