mod scene_node;
mod material;
mod geometry;
mod texture;

use std::fmt::Debug;
use crate::FSize;
use crate::maths::vectors::V3;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AABB {
    pub min: V3,
    pub max: V3,
}


pub trait Texture: Send + Sync + Debug {
    fn evaluate(&self, uv: (FSize, FSize)) -> V3;
    fn clone_box(&self) -> Box<dyn Texture + Send + Sync>;
}

impl Clone for Box<dyn Texture + Send + Sync> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone, Debug)]
pub struct SolidColorTexture {
    pub color: V3,
}

impl Texture for SolidColorTexture {
    fn evaluate(&self, _uv: (FSize, FSize)) -> V3 {
        self.color
    }

    fn clone_box(&self) -> Box<dyn Texture + Send + Sync> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct ImageTexture {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<V3>, // Simplified raster storage
}

impl Texture for ImageTexture {
    fn evaluate(&self, uv: (FSize, FSize)) -> V3 {
        // Simple UV lookup stub
        let x = (uv.0 * (self.width as FSize - 1.0)) as usize;
        let y = (uv.1 * (self.height as FSize - 1.0)) as usize;
        let idx = y * self.width as usize + x;
        self.pixels.get(idx).copied().unwrap_or(V3::ZERO)
    }

    fn clone_box(&self) -> Box<dyn Texture + Send + Sync> {
        Box::new(self.clone())
    }
}

