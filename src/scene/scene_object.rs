use crate::maths::vector::V3;
use crate::scene::{
    geometry::Geometry, material::Material, scene_node::SceneNode, texture::Texture,
};

#[derive(Debug, Clone)]
pub struct SceneObject {
    pub geometry: Geometry,
    pub material: Material,
    /// Radiance emitted by the surface (lights are ordinary objects with emission).
    pub emission: Option<Texture>,
    /// Alternative light spec: total emitted power (Tungsten "power").
    pub power: Option<V3>,
}

impl SceneObject {
    pub fn new(geometry: Geometry, material: Material) -> Self {
        SceneObject {
            geometry,
            material,
            emission: None,
            power: None,
        }
    }

    pub fn with_emission(mut self, emission: Texture) -> Self {
        self.emission = Some(emission);
        self
    }

    pub fn scene_node(self) -> SceneNode {
        SceneNode::new_object(self)
    }
}
