class_name GetTextureCommand
extends Command

func execute(actor: Node) -> void:
	if actor.has_method("get_texture"):
		var texture_rect = actor.get_parent().get_node("TextureRect")
		actor.get_texture(texture_rect)
	else:
		push_error("Actor does not have get_texture method")
