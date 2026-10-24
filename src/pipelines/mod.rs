use std::thread;

use crate::pipelines::{
    buffer::Buffer,
    render_config::{MultiFrameConfig, PipelineConfig},
};

pub mod buffer;
pub mod raytracer;
pub mod render_config;

mod shared;

fn render_image_internal(
    _pipeline_config: PipelineConfig,
    _multi_frame_config: MultiFrameConfig,
    _buffer: Buffer,
) {
    todo!()
}

pub fn render_image(
    pipeline_config: PipelineConfig,
    multi_frame_config: MultiFrameConfig,
) -> Buffer {
    let buffer = Buffer::new(1, 2);
    render_image_internal(pipeline_config, multi_frame_config, buffer.clone());
    buffer
}

pub fn render_image_background<F>(
    pipeline_config: PipelineConfig,
    multi_frame_config: MultiFrameConfig,
    on_complete: Option<F>,
) -> Buffer
where
    F: Fn(Buffer) + Send + 'static,
{
    let buffer = Buffer::new(1, 2);
    let buffer_clone = buffer.clone();

    thread::spawn(move || {
        render_image_internal(pipeline_config, multi_frame_config, buffer_clone.clone());
        if let Some(on_complete) = on_complete {
            on_complete(buffer_clone);
        }
    });

    buffer
}
