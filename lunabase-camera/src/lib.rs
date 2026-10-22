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
#[class(base=Node2D)]

struct CameraStream {
    base: Base<Node2D>,
    shutoff: Arc<AtomicBool>,
    stream_data: Arc<Mutex<Option<Vec<u8>>>>,
    width: Arc<Mutex<Option<u32>>>,
    height: Arc<Mutex<Option<u32>>>,
}
#[godot_api]
impl INode2D for CameraStream {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            stream_data: Arc::new(Mutex::new(None)),
            shutoff: Arc::new(AtomicBool::new(false)),
            width: Arc::new(Mutex::new(None)),
            height: Arc::new(Mutex::new(None)),
        }
    }
}

#[godot_api]
impl CameraStream {
    #[func]
    fn connect_camera(&mut self, address: GString, width: i32, height: i32) {
        let url = format!("tcp://{}", address);
        let texture = Arc::clone(&self.stream_data);
        let stream_data = Arc::clone(&self.stream_data);

        if width <= 0 || height <= 0 {
            return;
        }
        let width_shared = Arc::clone(&self.width);

        let height_shared = Arc::clone(&self.height);

        let w = width as u32;
        let h = height as u32;

        let shutoff = Arc::clone(&self.shutoff);

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
                if !shutoff.load(Ordering::Relaxed) {
                    break 'packets;
                }

                if (stream.index() != video_stream_index) {
                    continue;
                }
                if decoder.send_packet(&packet).is_err() {
                    continue;
                }
                let mut frame = ffmpeg::util::frame::video::Video::empty();
                let mut scaler: Option<ffmpeg::software::scaling::Context> = None;
                let mut newest_pts: i64 = i64::MIN; //unused but might be useful later

                while decoder.receive_frame(&mut frame).is_ok() {
                    let src_w = frame.width();
                    let src_h = frame.height();

                    let _scaler = scaler.get_or_insert_with(|| {
                        ffmpeg::software::scaling::Context::get(
                            frame.format(),
                            src_w,
                            src_h,
                            ffmpeg::format::Pixel::RGB24,
                            w,
                            h,
                            ffmpeg::software::scaling::Flags::BILINEAR,
                        )
                        .expect("Failed to create scaler")
                    });

                    let mut rgb_frame = ffmpeg::frame::Video::empty();
                    _scaler
                        .run(&frame, &mut rgb_frame)
                        .expect("Failed to scale");

                    let data = rgb_frame.data(0);
                    let stride = rgb_frame.stride(0);
                    let width = rgb_frame.width() as usize;
                    let height = rgb_frame.height() as usize;
                    let row_bytes = width * 3;

                    let mut packed_data = Vec::with_capacity(row_bytes * height);

                    for y in 0..height {
                        let start = y * stride;
                        let end = start + row_bytes;

                        packed_data.extend_from_slice(&data[start..end]);
                    }

                    *stream_data.lock().unwrap() = Some(packed_data.to_vec());
                    *width_shared.lock().unwrap() = Some(width as u32);
                    *height_shared.lock().unwrap() = Some(height as u32);
                }
            }
        });
    }
    #[func]
    fn toggle_cam(&mut self, address: GString, width: i32, height: i32) {
        let mut shutoff_val = self.shutoff.load(Ordering::Relaxed);
        shutoff_val = !shutoff_val;
        self.shutoff.store(shutoff_val, Ordering::Relaxed);
        if shutoff_val == true {
            self.connect_camera(address, width, height);
        }
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

        let Some(data) = stream_data_gd.as_ref() else {
            return;
        };

        let image = Image::create_from_data(
            width_gd as i32,
            height_gd as i32,
            false,
            godot::classes::image::Format::RGB8,
            &PackedByteArray::from(data.as_slice()),
        )
        .expect("Failed to create image");

        let texture: Gd<ImageTexture> = ImageTexture::create_from_image(&image).unwrap();
        texture_rect.set_texture(&texture);
    }
}
