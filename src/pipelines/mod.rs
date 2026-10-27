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
    print_progress: bool,
) {
    pipeline_config.render_pipeline.clone()(
        pipeline_config.render_config,
        scene,
        buffer,
        print_progress,
    );
}

pub fn render_image(
    pipeline_config: PipelineConfig,
    scene: SceneNode,
    multi_frame_config: MultiFrameConfig,
    print_progress: bool,
) -> Buffer {
    let buffer = Buffer::from_resolution(pipeline_config.render_config.resolution);
    render_image_internal(
        pipeline_config,
        scene,
        multi_frame_config,
        buffer.clone(),
        print_progress,
    );
    buffer
}

pub fn render_image_background<F>(
    pipeline_config: PipelineConfig,
    scene: SceneNode,
    multi_frame_config: MultiFrameConfig,
    on_complete: Option<F>,
    print_progress: bool,
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
            print_progress,
        );
        if let Some(on_complete) = on_complete {
            on_complete(buffer_clone);
        }
    });

    buffer
}
