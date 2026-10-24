use std::fmt::Debug;

use crate::{FSize, maths::vector::V3};

pub mod image;
pub mod solid_colour;

pub trait Texture: Send + Sync + Debug {
    fn evaluate(&self, uv: (FSize, FSize)) -> V3;

    fn clone_box(&self) -> Box<dyn Texture + Send + Sync>;
}

impl Clone for Box<dyn Texture + Send + Sync> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
