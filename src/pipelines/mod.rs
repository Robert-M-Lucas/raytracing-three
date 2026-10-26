use std::thread;

use crate::{
    pipelines::{
        buffer::Buffer,
        render_config::{MultiFrameConfig, PipelineConfig},
    },
    scene::scene_node::SceneNode,
};

pub mod buffer;
pub mod raytracer;
pub mod render_config;

pub mod shared;

fn render_image_internal(
    pipeline_config: PipelineConfig,
    scene: SceneNode,
    _multi_frame_config: MultiFrameConfig,
    buffer: Buffer,
) {
    pipeline_config.render_pipeline.clone()(pipeline_config.render_config, scene, buffer);
}

pub fn render_image(
    pipeline_config: PipelineConfig,
    scene: SceneNode,
    multi_frame_config: MultiFrameConfig,
) -> Buffer {
    let buffer = Buffer::from_resolution(pipeline_config.render_config.resolution);
    render_image_internal(pipeline_config, scene, multi_frame_config, buffer.clone());
    buffer
}

pub fn render_image_background<F>(
    pipeline_config: PipelineConfig,
    scene: SceneNode,
    multi_frame_config: MultiFrameConfig,
    on_complete: Option<F>,
) -> Buffer
where
    F: Fn(Buffer) + Send + 'static,
{
    let buffer = Buffer::from_resolution(pipeline_config.render_config.resolution);
    let buffer_clone = buffer.clone();

    thread::spawn(move || {
        render_image_internal(
            pipeline_config,
            scene,
            multi_frame_config,
            buffer_clone.clone(),
        );
        if let Some(on_complete) = on_complete {
            on_complete(buffer_clone);
        }
    });

    buffer
}
