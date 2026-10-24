use rayon::prelude::*;
use std::{thread, time::Duration};

use crate::{
    pipelines::{
        buffer::Buffer,
        render_config::{GlobalRenderConfig, RenderPipeline},
    },
    scene::scene_node::SceneNode,
};

#[derive(Debug, Clone)]
pub struct RaytracerConfig {}

pub fn raytracing_pipeline(config: RaytracerConfig) -> RenderPipeline {
    Box::new(move |grc, scene, buffer| {
        raytrace(grc, config, scene, buffer);
    })
}

fn raytrace(_grc: GlobalRenderConfig, _config: RaytracerConfig, scene: SceneNode, buffer: Buffer) {
    let _camera = scene
        .find_first_camera()
        .expect("Scene must contain a camera");

    let pixel_group = (buffer.width() * buffer.height() * 8) / num_cpus::get();

    (0..(buffer.width() * buffer.height()))
        .into_par_iter()
        .chunks(pixel_group)
        .for_each(|chunk| {
            for index in chunk {
                let x = index % buffer.width();
                let y = index / buffer.width();

                buffer.set_pixel(
                    x,
                    y,
                    if (x / 4 + y / 4).is_multiple_of(2) {
                        [125, 125, 125, 255]
                    } else {
                        [255, 255, 255, 255]
                    },
                );
            }
        });
}
