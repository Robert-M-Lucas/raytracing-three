use crate::pipelines::render_config::RenderPipeline;

#[derive(Debug, Clone)]
pub struct RaytracerConfig {}

pub fn raytracing_pipeline(config: RaytracerConfig) -> RenderPipeline {
    Box::new(move |_grc, _sn, _b| {
        println!("{config:?}");
    })
}
