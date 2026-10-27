#![allow(dead_code)]
#![allow(clippy::needless_range_loop)]

use crate::cli::{TransformedCliArgs, parse_args};
use crate::interface::{Interface, save_buffer_png};
use crate::pipelines::render_image;

mod cli;
mod coded_scenes;
mod interface;
mod loaders;
mod maths;
mod pipelines;
mod resolutions;
mod scene;

pub type FSize = f32;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustomUnwrapBehaviour {
    Panic,
    Default,
}
pub const CUSTOM_UNWRAP_BEHAVIOUR: CustomUnwrapBehaviour = CustomUnwrapBehaviour::Panic;

fn main() {
    let config = parse_args();
    let TransformedCliArgs {
        screenshot_config,
        preview_config,
        scene,
        multi_frame_config,
    } = config;

    if let Some((resolution, preview_config)) = preview_config {
        Interface::new(
            preview_config,
            Some(screenshot_config),
            multi_frame_config,
            scene,
            resolution,
        )
        .start();
    } else {
        save_buffer_png(&render_image(
            screenshot_config,
            scene,
            multi_frame_config,
            true,
        ));
    }
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use crate::{pipelines::render_image, resolutions::_4K_SQ};

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
        let buffer = render_image(perf_pipeline_config, scene, multi_frame_config, false);
        black_box(buffer);
    }
}
