use std::fmt::Debug;
use crate::FSize;
use crate::scene::{Texture, V3};

pub trait Material: Send + Sync + Debug {
    fn clone_box(&self) -> Box<dyn Material + Send + Sync>;
}

impl Clone for Box<dyn Material + Send + Sync> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone, Debug)]
pub struct LambertianMaterial {
    pub albedo_texture: Box<dyn Texture + Send + Sync>,
}

impl Material for LambertianMaterial {
    fn clone_box(&self) -> Box<dyn Material + Send + Sync> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct MetalMaterial {
    pub albedo: V3,
    pub roughness: FSize,
}

impl Material for MetalMaterial {
    fn clone_box(&self) -> Box<dyn Material + Send + Sync> {
        Box::new(self.clone())
    }
}
