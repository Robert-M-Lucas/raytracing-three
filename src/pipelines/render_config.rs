use std::fmt::Debug;

use dyn_clone::DynClone;

use crate::{FSize, pipelines::buffer::Buffer, scene::scene_node::SceneNode, utils::Resolution};

// (Scene, delta, time)
pub trait SendTimestepFn: Fn(SceneNode, FSize, FSize) -> SceneNode + Send + DynClone {}
impl<T: Fn(SceneNode, FSize, FSize) -> SceneNode + Send + Clone> SendTimestepFn for T {}

pub type TimestepFunction = Box<dyn SendTimestepFn>;

impl Clone for TimestepFunction {
    fn clone(&self) -> Self {
        dyn_clone::clone_box(self)
    }
}

impl Debug for TimestepFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TimestepFunction")
    }
}

pub fn do_nothing_timestep() -> TimestepFunction {
    Box::new(|sn, _delta, _time| sn)
}

pub trait SendPipelineFn: Fn(GlobalRenderConfig, SceneNode, Buffer) + Send + DynClone {}
impl<T: Fn(GlobalRenderConfig, SceneNode, Buffer) + Send + Clone> SendPipelineFn for T {}

pub type RenderPipeline = Box<dyn SendPipelineFn>;

impl Clone for RenderPipeline {
    fn clone(&self) -> Self {
        dyn_clone::clone_box(self)
    }
}

impl Debug for RenderPipeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RenderPipeline")
    }
}

#[derive(Debug, Clone)]
pub struct GlobalRenderConfig {
    pub resolution: Resolution,
}

#[derive(Clone)]
pub struct PipelineConfig {
    pub render_config: GlobalRenderConfig,
    pub render_pipeline: RenderPipeline,
}

#[derive(Debug, Clone)]
pub struct MultiFrameConfig {
    pub frames: usize,
    pub frame_timestep: FSize,
    pub scene_timestep_function: TimestepFunction,
}

impl MultiFrameConfig {
    pub fn none() -> MultiFrameConfig {
        MultiFrameConfig {
            frames: 1,
            frame_timestep: 1 as FSize,
            scene_timestep_function: do_nothing_timestep(),
        }
    }
}
