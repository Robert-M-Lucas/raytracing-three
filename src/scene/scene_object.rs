use crate::scene::{geometry::Geometry, material::Material, scene_node::SceneNode};

#[derive(Debug, Clone)]
pub struct SceneObject {
    geometry: Box<dyn Geometry + Send + Sync>,
    material: Box<dyn Material + Send + Sync>,
}

impl SceneObject {
    pub fn new(
        geometry: Box<dyn Geometry + Send + Sync>,
        material: Box<dyn Material + Send + Sync>,
    ) -> Self {
        SceneObject { geometry, material }
    }

    pub fn scene_node(self) -> SceneNode {
        SceneNode::new_object(self)
    }
}
