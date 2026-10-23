use rand::rngs::ThreadRng;
use crate::FSize;
use crate::interface::PixelArena;
use crate::maths::vectors::V3;
use crate::rendering::render_config::RenderConfig;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Camera {
    pub position: V3,
    pub rotation: (FSize, FSize), // Y, Z
    pub fov: FSize,
}

#[allow(dead_code)]
impl Camera {
    pub fn new(position: &V3, rotation: (FSize, FSize), fov: FSize) -> Self {
        Self {
            position: position.clone(),
            rotation,
            fov,
        }
    }

    pub fn get_image_threaded_old(&self, render_config: &RenderConfig) -> Vec<u8> {
        todo!()
    }

    pub fn get_image_threaded_buffer(
        &self,
        render_config: &RenderConfig,
        is_screenshot: bool,
    ) -> PixelArena {
        todo!()
    }

    pub fn get_image_threaded(
        &self,
        render_config: &RenderConfig,
        is_screenshot: bool,
        buffer: PixelArena,
    ) -> Vec<u8> {
        todo!()
    }

    pub fn get_image_sub_threaded(
        cam: Camera,
        render_config: &RenderConfig,
        is_screenshot: bool,
        chunk: &mut [u8],
        offset: usize,
        thread_id: u32,
    ) {
        todo!()
    }

    pub fn get_image(
        &self,
        render_config: &RenderConfig,
        rng: &mut ThreadRng,
        is_screenshot: bool,
        verbose: bool,
    ) -> Vec<u8> {
        vec![0; if is_screenshot {render_config.screenshot_resolution.0 * render_config.screenshot_resolution.1} else {render_config.resolution.0 * render_config.resolution.1} as usize * 3]
    }
}
