#![allow(dead_code)]

use itertools::Itertools;

use crate::coded_scenes::default_scene;
use crate::interface::Interface;
use crate::loaders::tungsten::load_tungsten_scene;
use crate::maths::lines::Line;
use crate::maths::vector::V3;
use crate::pipelines::raytracer::{RaytracerConfig, raytracing_pipeline};
use crate::pipelines::render_config::{GlobalRenderConfig, MultiFrameConfig, PipelineConfig};
use crate::pipelines::shared::bvh::BVH;
use crate::utils::{_8K_SQ, _144P, _720P_SQ};

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
            resolution: _144P,
        },
        render_pipeline: raytracing_pipeline(RaytracerConfig {}),
    };

    let screenshot_pipeline_config = PipelineConfig {
        render_config: GlobalRenderConfig { resolution: _8K_SQ },
        render_pipeline: raytracing_pipeline(RaytracerConfig {}),
    };

    let multi_frame_config = MultiFrameConfig::none();

    // let load_path = "static/scenes/cornell-box";
    // let scene = load_tungsten_scene(load_path).unwrap();
    let load_path = "hard-coded";
    let scene = default_scene();

    // let bvh = BVH::from_scene(scene);
    // println!("{:#?}", bvh.intersect(Line::new(V3::ZERO, V3::FORWARD), 0.001 as FSize, FSize::INFINITY).collect_vec());

    // println!("{:#?}", bvh);

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
