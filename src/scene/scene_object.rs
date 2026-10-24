use crate::maths::vector::V3;
use crate::scene::{
    geometry::Geometry, material::Material, scene_node::SceneNode, texture::Texture,
};

#[derive(Debug, Clone)]
pub struct SceneObject {
    pub name: Option<String>,
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
            name: None,
            geometry,
            material,
            emission: None,
            power: None,
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_emission(mut self, emission: Texture) -> Self {
        self.emission = Some(emission);
        self
    }

    pub fn scene_node(self) -> SceneNode {
        SceneNode::new_object(self)
    }
}
