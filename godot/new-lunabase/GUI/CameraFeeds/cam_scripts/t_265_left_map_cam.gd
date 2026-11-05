extends CameraStream

var is_streaming: bool = false
var connection_address:= "192.168.0.100:5001"


func _on_t_265_left_map_shutoff_toggled(toggled_on: bool) -> void:
	is_streaming = toggled_on
	self.toggle_cam(connection_address,340,200)


func _on_t_265_left_map_text_field_text_changed(new_text: String) -> void:
		connection_address = new_text
	
