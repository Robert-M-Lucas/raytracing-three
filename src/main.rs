#![allow(dead_code)]

use crate::interface::Interface;
use crate::pipelines::raytracer::{RaytracerConfig, raytracing_pipeline};
use crate::pipelines::render_config::{
    GlobalRenderConfig, MultiFrameConfig, PipelineConfig,
};
use crate::scene::camera::Camera;
use crate::scene::geometry::Geometry;
use crate::scene::geometry::sphere::Sphere;
use crate::scene::material::{LambertianMaterial, Material};
use crate::scene::scene_node::SceneNode;
use crate::scene::scene_object::SceneObject;
use crate::scene::texture::Texture;
use crate::scene::texture::solid_colour::SolidColorTexture;
use crate::utils::{_240P, _720P, _1080P};

mod interface;
mod maths;
mod pipelines;
mod scene;
mod utils;

pub type FSize = f32;

fn main() {
    let pipeline_config = PipelineConfig {
        render_config: GlobalRenderConfig {
            resolution: _240P,
        },
        render_pipeline: raytracing_pipeline(RaytracerConfig {}),
    };

    let _screenshot_pipeline_config = PipelineConfig {
        render_config: GlobalRenderConfig {
            resolution: _1080P,
        },
        render_pipeline: raytracing_pipeline(RaytracerConfig {}),
    };

    let multi_frame_config = MultiFrameConfig::none();

    let scene = SceneNode::new_empty()
        .with_child(Camera::default().scene_node())
        .with_child(
            SceneObject::new(
                Sphere { radius: 5 as FSize }.clone_box(),
                LambertianMaterial {
                    albedo_texture: SolidColorTexture { color: v![1, 0, 0] }.clone_box()
                }.clone_box()
                ,
            )
            .scene_node(),
        );

    Interface::new(pipeline_config, multi_frame_config, scene, _720P).start();
}
