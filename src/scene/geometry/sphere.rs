use crate::{
    FSize,
    maths::vector::V3,
    scene::{geometry::Geometry},
};

#[derive(Clone, Debug)]
pub struct Sphere {
    pub radius: FSize,
}

impl Geometry for Sphere {
    fn clone_box(&self) -> Box<dyn Geometry + Send + Sync> {
        Box::new(self.clone())
    }
}
