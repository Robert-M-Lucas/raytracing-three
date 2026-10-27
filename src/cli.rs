use std::path::PathBuf;

use crate::{
    coded_scenes::default_scene,
    loaders::tungsten::load_tungsten_scene,
    pipelines::render_config::{MultiFrameConfig, PipelineConfig},
    resolutions::*,
    scene::scene_node::SceneNode,
};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    no_multithread: bool,
    #[arg(long = "scpath")]
    scene_path: Option<PathBuf>,
    #[arg(long="wres", value_parser = parse_resolution)]
    window_resolution: Option<Resolution>,
    #[arg(long="pres", value_parser = parse_resolution)]
    preview_resolution: Option<Resolution>,
    #[arg(long="sres", value_parser = parse_resolution, default_value="4ksq")]
    screenshot_resoltion: Resolution,
}

fn parse_resolution(s: &str) -> Result<Resolution, String> {
    let lower = s.to_lowercase();

    match lower.as_str() {
        "32k" => return Ok(_32K),
        "16k" => return Ok(_16K),
        "8k" => return Ok(_8K),
        "4k" => return Ok(_4K),
        "1440p" => return Ok(_1440P),
        "1080p" => return Ok(_1080P),
        "720p" => return Ok(_720P),
        "480p" => return Ok(_480P),
        "360p" => return Ok(_360P),
        "240p" => return Ok(_240P),
        "144p" => return Ok(_144P),

        "32ksq" => return Ok(_32K_SQ),
        "16ksq" => return Ok(_16K_SQ),
        "8ksq" => return Ok(_8K_SQ),
        "4ksq" => return Ok(_4K_SQ),
        "1440psq" => return Ok(_1440P_SQ),
        "1080psq" => return Ok(_1080P_SQ),
        "720psq" => return Ok(_720P_SQ),
        "480psq" => return Ok(_480P_SQ),
        "360psq" => return Ok(_360P_SQ),
        "240psq" => return Ok(_240P_SQ),
        "144psq" => return Ok(_144P_SQ),
        _ => {}
    }

    let parts: Vec<&str> = lower.split('x').collect();
    if parts.len() == 2 {
        let width = parts[0]
            .trim()
            .parse::<u32>()
            .map_err(|e| format!("Invalid width: {}", e))?;
        let height = parts[1]
            .trim()
            .parse::<u32>()
            .map_err(|e| format!("Invalid height: {}", e))?;
        return Ok((width, height));
    }

    Err(format!(
        "Invalid resolution '{}'. Expected a named preset (e.g., '720p', '720psq') or dimensions in WIDTHxHEIGHT format (e.g., '100x200').",
        s
    ))
}

pub struct TransformedCliArgs {
    pub screenshot_config: PipelineConfig,
    pub preview_config: Option<(Resolution, PipelineConfig)>,
    pub scene: SceneNode,
    pub multi_frame_config: MultiFrameConfig,
}

pub fn parse_args() -> TransformedCliArgs {
    let args = Args::parse();

    let scene = if let Some(scene_path) = &args.scene_path {
        load_tungsten_scene(scene_path)
            .unwrap()
            .with_name(scene_path.to_string_lossy())
    } else {
        default_scene()
    };

    let mut screenshot_config = PipelineConfig::default_screenshot_config();
    screenshot_config.render_config.resolution = args.screenshot_resoltion;
    screenshot_config.render_config.multithread = !args.no_multithread;

    let preview_config = if args.window_resolution.is_some() || args.preview_resolution.is_some() {
        // Implies a GUI
        let mut preview_config = PipelineConfig::default_preview_config();
        if let Some(preview_resolution) = &args.preview_resolution {
            preview_config.render_config.resolution = *preview_resolution;
        }

        preview_config.render_config.multithread = !args.no_multithread;

        Some((args.window_resolution.unwrap_or(_720P_SQ), preview_config))
    } else {
        None
    };

    TransformedCliArgs {
        screenshot_config,
        preview_config,
        scene,
        multi_frame_config: MultiFrameConfig::none(),
    }
}
