extends CameraStream

var is_streaming: bool = false
#var connection_address:= "192.168.0.100:4002"
var connection_address:= "127.0.0.1:4002"

func _on_front_cam_shutoff_toggled(toggled_on: bool) -> void:
	is_streaming = toggled_on
	self.toggle_cam(connection_address,340,200)
