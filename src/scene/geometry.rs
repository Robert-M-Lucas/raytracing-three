use std::fmt::Debug;
use crate::FSize;
use crate::scene::{V3, AABB};

pub trait Geometry: Send + Sync + Debug {
    fn bounding_box(&self) -> AABB;
    fn clone_box(&self) -> Box<dyn Geometry + Send + Sync>;
}

impl Clone for Box<dyn Geometry + Send + Sync> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone, Debug)]
pub struct Sphere {
    pub radius: FSize,
}

impl Geometry for Sphere {
    fn bounding_box(&self) -> AABB {
        AABB {
            min: V3::new_v3(-self.radius, -self.radius, -self.radius),
            max: V3::new_v3(self.radius, self.radius, self.radius),
        }
    }

    fn clone_box(&self) -> Box<dyn Geometry + Send + Sync> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct ObjMesh {
    pub name: String,
    pub vertices: Vec<V3>,
    pub normals: Vec<V3>,
    pub indices: Vec<u32>,
    pub cached_bounds: AABB,
}

impl Geometry for ObjMesh {
    fn bounding_box(&self) -> AABB {
        self.cached_bounds
    }

    fn clone_box(&self) -> Box<dyn Geometry + Send + Sync> {
        Box::new(self.clone())
    }
}