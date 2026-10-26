#![allow(dead_code)]
#![allow(clippy::needless_range_loop)]

use crate::interface::Interface;
use crate::loaders::tungsten::load_tungsten_scene;
use crate::pipelines::raytracer::{RaytracerConfig, raytracing_pipeline};
use crate::pipelines::render_config::{GlobalRenderConfig, MultiFrameConfig, PipelineConfig};
use crate::utils::{_8K_SQ, _16K, _16K_SQ, _240P_SQ, _720P_SQ};

mod coded_scenes;
mod interface;
mod loaders;
mod maths;
mod pipelines;
mod scene;
mod utils;

pub type FSize = f32;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustomUnwrapBehaviour {
    Panic,
    Default,
}
pub const CUSTOM_UNWRAP_BEHAVIOUR: CustomUnwrapBehaviour = CustomUnwrapBehaviour::Panic;

fn main() {
    let pipeline_config = PipelineConfig {
        render_config: GlobalRenderConfig {
            resolution: _240P_SQ,
            multithread: true,
        },
        render_pipeline: raytracing_pipeline(RaytracerConfig {}),
    };

    let screenshot_pipeline_config = PipelineConfig {
        render_config: GlobalRenderConfig {
            resolution: _16K_SQ,
            multithread: true,
        },
        render_pipeline: raytracing_pipeline(RaytracerConfig {}),
    };

    let multi_frame_config = MultiFrameConfig::none();

    let load_path = "static/scenes/kitchen";
    let scene = load_tungsten_scene(load_path).unwrap();
    // let load_path = "hard-coded";
    // let scene = default_scene();

    // let bvh = BVH::from_scene(scene);
    // let ray = Line::new(V3::ZERO, V3::FORWARD);
    // for ((transform, object), subindex) in bvh.intersect(ray, 0.001 as FSize, FSize::INFINITY).collect_vec() {
    //     println!("{} {:?}", transform, object);

    //     let x = match &object.geometry {
    //         scene::geometry::Geometry::Sphere(sphere) => intersect_sphere(&ray, sphere, transform),
    //         scene::geometry::Geometry::Quad => todo!(),
    //         scene::geometry::Geometry::Disk => todo!(),
    //         scene::geometry::Geometry::Cube => todo!(),
    //         scene::geometry::Geometry::Mesh(mesh) => todo!(),
    //     };
    //     println!("Hit: {:?}", x);
    // };

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

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use crate::{pipelines::render_image, utils::_4K_SQ};

    use super::*;
    #[test]
    fn single() {
        let perf_pipeline_config = PipelineConfig {
            render_config: GlobalRenderConfig {
                resolution: _4K_SQ,
                multithread: false,
            },
            render_pipeline: raytracing_pipeline(RaytracerConfig {}),
        };
        let multi_frame_config = MultiFrameConfig::none();
        let load_path = "static/scenes/kitchen";
        let scene = load_tungsten_scene(load_path).unwrap();
        let buffer = render_image(perf_pipeline_config, scene, multi_frame_config);
        black_box(buffer);
    }
}
