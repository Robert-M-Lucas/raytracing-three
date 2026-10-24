use crate::scene::AABB;
use std::fmt::Debug;

pub mod obj_mesh;
pub mod sphere;

pub trait Geometry: Send + Sync + Debug {
    fn bounding_box(&self) -> AABB;

    fn clone_box(&self) -> Box<dyn Geometry + Send + Sync>;
}

impl Clone for Box<dyn Geometry + Send + Sync> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
