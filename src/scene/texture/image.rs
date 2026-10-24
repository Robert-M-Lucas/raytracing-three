use crate::{FSize, maths::vector::V3, scene::texture::Texture};

#[derive(Clone, Debug)]
pub struct ImageTexture {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<V3>, // Needs to be arced
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
