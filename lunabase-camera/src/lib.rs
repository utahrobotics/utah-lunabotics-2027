use ffmpeg::software::scaling::Context;
use ffmpeg_next as ffmpeg;
use godot::classes::Image;
use godot::classes::ImageTexture;
use godot::classes::TextureRect;
use godot::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::{f32::consts::E, net::TcpStream};
struct LunabaseCamera {}
#[gdextension]
unsafe impl ExtensionLibrary for LunabaseCamera {}

#[derive(GodotClass)]
#[class(base=Node)]

struct CameraStream {
    base: Base<Node>,
    shutoff: bool,
    stream_data: Arc<Mutex<Option<Vec<u8>>>>,
    width: Arc<Mutex<Option<u32>>>,
    height: Arc<Mutex<Option<u32>>>,
}
#[godot_api]
impl INode for CameraStream {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            stream_data: Arc::new(Mutex::new(None)),
            shutoff: false,
            width: Arc::new(Mutex::new(None)),
            height: Arc::new(Mutex::new(None)),
        }
    }
}

#[godot_api]
impl CameraStream {
    #[func]
    fn connect_camera(&mut self, address: GString) {
        let url = format!("tcp://{}", address);
        let texture = Arc::clone(&self.stream_data);
        let stream_data = Arc::clone(&self.stream_data);
        let width_shared = Arc::clone(&self.width);
        let height_shared = Arc::clone(&self.height);
        thread::spawn(move || {
            ffmpeg::init().expect("Failed to init ffmpeg");
            let mut opts = ffmpeg::Dictionary::new();
            opts.set("timeout", "1000000");
            opts.set("fflags", "nobuffer+discardcorrupt");
            opts.set("flags", "low_delay");
            opts.set("max_delay", "0");
            // opts.set("probesize", "32");
            // opts.set("analyzeduration", "0");

            let mut input_ctx = match ffmpeg::format::input_with_dictionary(&url, opts.clone()) {
                Ok(ctx) => {
                    // godot_print!("success");
                    ctx
                }
                Err(_) => {
                    //godot_error!("Failed to connect to camera:");
                    return;
                }
            };

            let video_stream_index = match input_ctx.streams().best(ffmpeg::media::Type::Video) {
                Some(s) => s.index(),
                None => {
                    eprintln!("No video stream found in ");
                    return;
                }
            };
            let codec_params = input_ctx.stream(video_stream_index).unwrap().parameters();
            let mut decoder = ffmpeg::codec::Context::from_parameters(codec_params)
                .and_then(|c| c.decoder().video())
                .expect("Failed to open H.264 decoder");

            'packets: for (stream, packet) in input_ctx.packets() {
                if (stream.index() != video_stream_index) {
                    continue;
                }
                if decoder.send_packet(&packet).is_err() {
                    continue;
                }
                let mut frame = ffmpeg::util::frame::video::Video::empty();
                let mut scaler: Option<ffmpeg::software::scaling::Context> = None;
                let mut newest_pts: i64 = i64::MIN;
                while decoder.receive_frame(&mut frame).is_ok() {
                    let src_w = frame.width();
                    let src_h = frame.height();

                    let _scaler = scaler.get_or_insert_with(|| {
                        ffmpeg::software::scaling::Context::get(
                            frame.format(),
                            src_w,
                            src_h,
                            ffmpeg::format::Pixel::RGB24,
                            src_w,
                            src_h,
                            ffmpeg::software::scaling::Flags::BILINEAR,
                        )
                        .expect("Failed to create scaler")
                    });

                    let mut rgb_frame = ffmpeg::frame::Video::empty();
                    _scaler
                        .run(&frame, &mut rgb_frame)
                        .expect("Failed to scale");

                    let data = rgb_frame.data(0);
                    let width = rgb_frame.width();
                    let height = rgb_frame.height();
                    let stride = rgb_frame.stride(0);

                    *stream_data.lock().unwrap() = Some(data.to_vec());
                    *width_shared.lock().unwrap() = Some(width);
                    *height_shared.lock().unwrap() = Some(height);

                    //godot_print!("GOT A FRAME: {}x{}", width, height);
                    //break 'packets;
                }
            }
        });
    }
    #[func]
    fn shutoff_cam(&mut self) {
        self.shutoff = true;
    }
    #[func]
    fn turn_on_cam(&mut self) {
        self.shutoff = false;
    }
    #[func]
    fn get_texture(&self, mut texture_rect: Gd<TextureRect>) {
        let width_gd = match *self.width.lock().unwrap() {
            Some(width) => width,
            None => return,
        };
        let height_gd = match *self.height.lock().unwrap() {
            Some(height) => height,
            None => return,
        };
        let stream_data_gd = self.stream_data.lock().unwrap();
        let image = Image::create_from_data(
            width_gd as i32,
            height_gd as i32,
            false,
            godot::classes::image::Format::RGB8,
            &PackedByteArray::from(stream_data_gd.as_ref().unwrap().as_slice()),
        )
        .expect("Failed to create image");

        let texture: Gd<ImageTexture> = ImageTexture::create_from_image(&image).unwrap();
        texture_rect.set_texture(&texture);
    }
}
