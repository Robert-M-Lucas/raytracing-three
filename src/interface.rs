use crate::FSize;
use crate::pipelines::buffer::Buffer;
use crate::pipelines::render_config::{MultiFrameConfig, PipelineConfig};
use crate::pipelines::{render_image, render_image_background};
use crate::scene::scene_node::SceneNode;
use crate::utils::Resolution;
use chrono::{Datelike, Timelike};
use image::{DynamicImage, ImageBuffer, Rgba};
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::render::Canvas;
use sdl2::surface::Surface;
use sdl2::video::Window;
use std::fs;
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub struct Interface {
    title: String,
    pipeline_config: PipelineConfig,
    screenshot_config: Option<PipelineConfig>,
    multi_frame_config: MultiFrameConfig,
    scene: SceneNode,
    window_resolution: Resolution,
    render_progress: Arc<Mutex<Option<Buffer>>>,
}

impl Interface {
    pub fn new(
        title: String,
        pipeline_config: PipelineConfig,
        screenshot_config: Option<PipelineConfig>,
        multi_frame_config: MultiFrameConfig,
        scene: SceneNode,
        window_resolution: Resolution,
    ) -> Interface {
        Self {
            title,
            pipeline_config,
            screenshot_config,
            multi_frame_config,
            scene,
            window_resolution,
            render_progress: Arc::new(Mutex::new(None)),
        }
    }

    pub fn start(&mut self) {
        let (mut canvas, mut event_pump) = self.initialise_display();

        let mut now = Instant::now();

        'running: loop {
            let delta_time = now.elapsed().as_secs_f64() as FSize;
            now = Instant::now();

            let render_progress = self.render_progress.lock().unwrap();

            let render_progress_buffer = render_progress.clone();
            drop(render_progress);

            if let Some(display_override) = &render_progress_buffer {
                canvas
                    .window_mut()
                    .set_title(&format!("[SCREENSHOT] {}", self.title))
                    .unwrap();

                for _event in event_pump.poll_iter() {}

                let mut snapshot: Vec<u8> = display_override.to_rgba_vec();

                let surface = Surface::from_data(
                    snapshot.as_mut(),
                    display_override.width() as u32,
                    display_override.height() as u32,
                    4 * display_override.width() as u32,
                    sdl2::pixels::PixelFormatEnum::RGBA32,
                )
                .unwrap();

                canvas
                    .copy(
                        &surface.as_texture(&canvas.texture_creator()).unwrap(),
                        None,
                        None,
                    )
                    .unwrap();
            } else {
                canvas.window_mut().set_title(&self.title).unwrap();

                let exit = self.handle_input(&mut event_pump, delta_time);
                if exit {
                    break 'running;
                }
                if self.render_progress.lock().unwrap().is_some() {
                    continue;
                }

                let buffer = render_image(
                    self.pipeline_config.clone(),
                    self.scene.clone(),
                    MultiFrameConfig::none(),
                );

                let mut image = buffer.to_rgba_vec();

                let surface = Surface::from_data(
                    image.as_mut(),
                    buffer.width() as u32,
                    buffer.height() as u32,
                    4 * buffer.width() as u32,
                    sdl2::pixels::PixelFormatEnum::RGBA32,
                )
                .unwrap();

                canvas
                    .copy(
                        &surface.as_texture(&canvas.texture_creator()).unwrap(),
                        None,
                        None,
                    )
                    .unwrap();
            };

            canvas.present();
        }
    }

    fn initialise_display(&self) -> (Canvas<Window>, EventPump) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        println!(
            "Using video subsytem: {:?}",
            video_subsystem.current_video_driver()
        );

        sdl_context.mouse().show_cursor(false);

        let window = video_subsystem
            .window(
                &self.title,
                self.window_resolution.0,
                self.window_resolution.1,
            )
            .position_centered()
            .build()
            .unwrap();

        sdl_context.mouse().set_relative_mouse_mode(true);

        let canvas = window.into_canvas().accelerated().build().unwrap();

        let event_pump = sdl_context.event_pump().unwrap();

        (canvas, event_pump)
    }

    fn handle_input(&mut self, event_pump: &mut EventPump, delta_time: FSize) -> bool {
        let (mut look_x, mut look_y) = (0.0 as FSize, 0.0 as FSize);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => return true,
                Event::MouseMotion { xrel, yrel, .. } => {
                    look_x += xrel as FSize;
                    look_y += yrel as FSize;
                }
                _ => {}
            }
        }

        let keyboard_state = event_pump.keyboard_state();
        let down = |sc: Scancode| keyboard_state.is_scancode_pressed(sc);
        // +1 if any `pos` key is held, -1 if any `neg` key is held (cancel if both)
        let axis = |pos: &[Scancode], neg: &[Scancode]| -> FSize {
            (pos.iter().any(|&s| down(s)) as i32 - neg.iter().any(|&s| down(s)) as i32) as FSize
        };

        let fast = delta_time * 20.0;
        let slow = delta_time * 1.0;

        let right = fast * axis(&[Scancode::D], &[Scancode::A])
            + slow * axis(&[Scancode::Right], &[Scancode::Left]);
        let forward = fast * axis(&[Scancode::W], &[Scancode::S])
            + slow * axis(&[Scancode::Up], &[Scancode::Down]);
        let up = fast * axis(&[Scancode::Space], &[Scancode::C, Scancode::LCtrl])
            + slow * axis(&[Scancode::Q], &[Scancode::E]);

        if let Some(camera_node) = self.scene.find_first_camera_mut() {
            camera_node
                .transform
                .fps_look(-look_x / 1000.0, -look_y / 1000.0);
            camera_node.transform.fps_translate(right, up, forward);
        }

        if down(Scancode::M) {
            println!("Taking screenshot");

            let mut render_progress = self.render_progress.lock().unwrap(); // Lock early to preserve order

            if render_progress.is_some() {
                return false; // Render in progress
            }

            let render_progress_clone = self.render_progress.clone();

            let buffer = render_image_background(
                if let Some(screenshot_config) = &self.screenshot_config {
                    screenshot_config.clone()
                } else {
                    self.pipeline_config.clone()
                },
                self.scene.clone(),
                self.multi_frame_config.clone(),
                Some(move |buffer: Buffer| {
                    println!("Screenshot complete");
                    *render_progress_clone.lock().unwrap() = None;

                    let now = chrono::offset::Local::now();
                    let file_name = format!(
                        "Render {}-{}-{} {}-{}-{} {}x{}",
                        format!("{:0>2}", now.year()),
                        format!("{:0>2}", now.month()),
                        format!("{:0>2}", now.day()),
                        format!("{:0>2}", now.hour()),
                        format!("{:0>2}", now.minute()),
                        format!("{:0>2}", now.second()),
                        buffer.width(),
                        buffer.height()
                    );
                    let rgba = buffer.to_rgba_vec();
                    let img = DynamicImage::ImageRgba8(
                        ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(
                            buffer.width() as u32,
                            buffer.height() as u32,
                            rgba,
                        )
                        .unwrap(),
                    );

                    let mut is_err = fs::create_dir_all("renders").is_err();
                    if !is_err {
                        is_err = img.save(format!("renders/{}.png", file_name)).is_err();
                    }
                    if is_err {
                        println!("Saving file to 'renders/{}.png' failed!", file_name);
                    } else {
                        println!("Saved file to 'renders/{}.png'", file_name);
                    }
                }),
            );

            *render_progress = Some(buffer);
        }

        false
    }
}
