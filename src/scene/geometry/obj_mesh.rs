use crate::{
    maths::vector::V3,
    scene::{AABB, geometry::Geometry},
};

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
