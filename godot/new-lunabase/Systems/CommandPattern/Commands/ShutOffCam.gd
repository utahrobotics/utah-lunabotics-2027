class_name ShutOffCamCommand
extends Command


func execute(actor: Node) -> void:
	if actor.has_method("shutoff_cam"):
		actor.shutoff_cam
	else:
		push_error("Actor does not have shutoff_cam method")
