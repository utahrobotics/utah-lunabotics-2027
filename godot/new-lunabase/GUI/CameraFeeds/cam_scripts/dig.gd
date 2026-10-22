extends CameraStream

var is_streaming: bool = false
var connection_address:= "192.168.0.100:4000"


func _on_dig_toggle_toggled(toggled_on: bool) -> void:
	is_streaming = toggled_on
	self.toggle_cam(connection_address,340,200)
