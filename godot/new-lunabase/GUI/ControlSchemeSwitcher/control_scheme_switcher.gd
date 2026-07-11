class_name ControlSchemeSwitcher
extends Control

@onready var touch_controls_check: CheckButton = $VBoxContainer/TouchControlsRow/TouchControlsCheckButton
@onready var status_label: Label = $VBoxContainer/ConfigStatusLabel

const ControlConfigLoaderScript = preload("res://Systems/ControlSchemes/control_config_loader.gd")

signal updated_control_scheme


func _ready() -> void:
	_reload_config_now()
	sync_touch_controls_from_settings()


func sync_touch_controls_from_settings() -> void:
	if touch_controls_check:
		touch_controls_check.set_pressed_no_signal(SettingsMenu.touch_screen_controls_enabled)


func _on_touch_controls_check_toggled(pressed: bool) -> void:
	SettingsMenu.set_touch_screen_controls(pressed)


func _on_reload_config_button_pressed() -> void:
	_reload_config_now()


func _reload_config_now() -> void:
	ControlConfigLoaderScript.ensure_loaded()
	var human_controller: Node = get_tree().root.find_child("LunabaseHumanController", true, false)
	if human_controller != null:
		ControlConfigLoaderScript.apply_controller_flags(human_controller)
	if status_label:
		status_label.text = "Controls loaded from %s" % _get_config_path_label()
	updated_control_scheme.emit()


func _get_config_path_label() -> String:
	if OS.has_feature("editor"):
		return "user://control_config.json"
	return "control_config.json next to the executable"
