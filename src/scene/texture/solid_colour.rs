use crate::{FSize, maths::vector::V3, scene::texture::Texture};

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
