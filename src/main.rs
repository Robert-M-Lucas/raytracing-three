#![allow(dead_code)]

use crate::interface::Interface;
use crate::loaders::tungsten::load_tungsten_scene;
use crate::pipelines::raytracer::{RaytracerConfig, raytracing_pipeline};
use crate::pipelines::render_config::{GlobalRenderConfig, MultiFrameConfig, PipelineConfig};
use crate::scene::camera::Camera;
use crate::scene::geometry::{Geometry, Sphere};
use crate::scene::material::Material;
use crate::scene::scene_node::SceneNode;
use crate::scene::scene_object::SceneObject;
use crate::scene::texture::Texture;
use crate::utils::{_4K, _8K, _16K, _32K, _144P, _720P, _1080P};

mod interface;
mod loaders;
mod maths;
mod pipelines;
mod scene;
mod utils;

pub type FSize = f32;

fn main() {
    let pipeline_config = PipelineConfig {
        render_config: GlobalRenderConfig { resolution: _144P },
        render_pipeline: raytracing_pipeline(RaytracerConfig {}),
    };

    let screenshot_pipeline_config = PipelineConfig {
        render_config: GlobalRenderConfig { resolution: _32K },
        render_pipeline: raytracing_pipeline(RaytracerConfig {}),
    };

    let multi_frame_config = MultiFrameConfig::none();

    let load_path = "Hard-coded";
    let scene = SceneNode::new_empty()
        .with_child(Camera::default().scene_node())
        .with_child(
            SceneObject::new(
                Geometry::Sphere(Sphere { radius: 5 as FSize }),
                Material::lambert(Texture::Solid(v![1.0, 0.0, 0.0]))
            )
            .scene_node(),
        );

    // let load_path = "static/scenes/cornell-box";
    // let scene = load_tungsten_scene(load_path).unwrap();

    Interface::new(
        load_path.to_string(),
        pipeline_config,
        Some(screenshot_pipeline_config),
        multi_frame_config,
        scene,
        _720P,
    )
    .start();
}
