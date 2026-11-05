extends CameraStream

var is_streaming: bool = false
var connection_address:= "192.168.0.100:4002"


func _on_front_cam_shutoff_toggled(toggled_on: bool) -> void:
	is_streaming = toggled_on
	self.toggle_cam(connection_address,340,200)
	print(connection_address)


func _on_line_edit_text_changed(new_text: String) -> void:
	connection_address = new_text
	
