extends CameraStream

var is_streaming: bool = false
var port: String = "4002"
var connection_address: String
@onready var prefix := $FrontPrefix

func _ready() -> void:
	connection_address = get_parent().connection_prefix + port
	prefix.text = get_parent().connection_prefix
	

func _on_front_cam_shutoff_toggled(toggled_on: bool) -> void:
	is_streaming = toggled_on
	self.toggle_cam(connection_address,340,200)



func _on_line_edit_text_changed(new_text: String) -> void:
	port = new_text
	connection_address = get_parent().connection_prefix +":" + port
	
