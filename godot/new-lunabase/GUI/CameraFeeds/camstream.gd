extends Node2D
@onready var FrontCam = $FrontCam
@onready var FrontCamImg = $FrontCam/FrontCamImage

@onready var ObstacleMapcam = $ObstacleMapCam
@onready var ObstacleMapCamImg = $ObstacleMapCam/ObstacleMapImage

@onready var T265LeftMapCam = $T265LeftMapCam
@onready var T265LeftMapCamImg = $T265LeftMapCam/T265LeftMapCam

@onready var RealsenseFront = $RealsenseFront
@onready var RealsenseFrontImg = $RealsenseFront/RealsenseFront

@onready var T265Front = $T265Front
@onready var T265FrontImg = $T265Front/T265FrontImg

@onready var Dig = $Dig
@onready var DigImg = $Dig/DigImg
var cameras
func _ready()-> void:
	cameras = {
	FrontCam: FrontCamImg,
	ObstacleMapcam: ObstacleMapCamImg, 
	T265LeftMapCam: T265LeftMapCamImg,
	RealsenseFront: RealsenseFrontImg,
	T265Front: T265FrontImg,
	Dig: DigImg
}

func _process(delta: float) -> void:
	for cam in cameras:
		if cam.is_streaming == true:
			var texture = cameras[cam]
			cam.get_texture(texture)
			

func _on_connect_all_butt_toggled(toggled_on: bool) -> void:
	if toggled_on == true:
		print("BUTTON PRESSED")
		#cam_stream.connect_camera("192.168.0.100:4002")
		for cam in cameras:
			if cam.is_streaming == true:
				continue
			cam.toggle_cam(cam.connection_address,340,240)
