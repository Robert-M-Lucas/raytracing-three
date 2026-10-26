#![allow(dead_code)]

use crate::interface::Interface;
use crate::loaders::tungsten::load_tungsten_scene;
use crate::pipelines::raytracer::{RaytracerConfig, raytracing_pipeline};
use crate::pipelines::render_config::{GlobalRenderConfig, MultiFrameConfig, PipelineConfig};
use crate::utils::{_8K_SQ, _720P_SQ};

mod coded_scenes;
mod interface;
mod loaders;
mod maths;
mod pipelines;
mod scene;
mod utils;

pub type FSize = f32;

fn main() {
    let pipeline_config = PipelineConfig {
        render_config: GlobalRenderConfig {
            resolution: _720P_SQ,
        },
        render_pipeline: raytracing_pipeline(RaytracerConfig {}),
    };

    let screenshot_pipeline_config = PipelineConfig {
        render_config: GlobalRenderConfig { resolution: _8K_SQ },
        render_pipeline: raytracing_pipeline(RaytracerConfig {}),
    };

    let multi_frame_config = MultiFrameConfig::none();

    let load_path = "static/scenes/cornell-box";
    let scene = load_tungsten_scene(load_path).unwrap();

    Interface::new(
        load_path.to_string(),
        pipeline_config,
        Some(screenshot_pipeline_config),
        multi_frame_config,
        scene,
        _720P_SQ,
    )
    .start();
}
