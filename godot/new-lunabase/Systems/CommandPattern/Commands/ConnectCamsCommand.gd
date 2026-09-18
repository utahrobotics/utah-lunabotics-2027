class_name ConnectCamsCommand
extends Command

@export var address: String = "127.0.0.1:4002"

func execute(actor: Node) -> void:
	if actor.has_method("connect_camera"):
		actor.connect_camera(address)
	else:
		push_error("Actor does not have connect_camera method")
