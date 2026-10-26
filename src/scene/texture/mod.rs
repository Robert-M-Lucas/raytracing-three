use std::{path::PathBuf, sync::Arc};

use crate::{FSize, maths::vector::V3};

/// Decoded image data. Always shared behind an `Arc` so cloning a scene is cheap.
/// To edit pixels in place use `Arc::make_mut` (copy-on-write).
#[derive(Clone, Debug)]
pub struct ImageData {
    pub source: PathBuf,
    pub width: u32,
    pub height: u32,
    /// Row-major, top row first, linear RGB
    pub pixels: Vec<V3>,
}

#[derive(Clone, Debug)]
pub enum Texture {
    Solid(V3),
    Image(Arc<ImageData>),
}

impl Texture {
    pub fn grey(v: FSize) -> Self {
        Texture::Solid(V3::new_v3(v, v, v))
    }

    /// `uv` wraps (repeat); v = 0 is the bottom of the image.
    pub fn evaluate(&self, uv: (FSize, FSize)) -> V3 {
        match self {
            Texture::Solid(c) => *c,
            Texture::Image(img) => {
                let u = uv.0 - uv.0.floor();
                let v = 1.0 - (uv.1 - uv.1.floor());
                let x = ((u * img.width as FSize) as usize).min(img.width as usize - 1);
                let y = ((v * img.height as FSize) as usize).min(img.height as usize - 1);
                img.pixels[y * img.width as usize + x]
            }
        }
    }
}
