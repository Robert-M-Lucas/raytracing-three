use crate::maths::matrices::M4;
use crate::scene::geometry::Geometry;
use crate::scene::material::Material;

#[derive(Clone, Debug)]
pub struct SceneNode {
    pub name: String,
    pub transform: M4,
    pub geometry: Option<Box<dyn Geometry + Send + Sync>>,
    pub material: Option<Box<dyn Material + Send + Sync>>,
    pub children: Vec<SceneNode>,
}

impl SceneNode {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            transform: M4::default(),
            geometry: None,
            material: None,
            children: Vec::new(),
        }
    }

    pub fn with_geometry(mut self, geometry: impl Geometry + Send + Sync + 'static) -> Self {
        self.geometry = Some(Box::new(geometry));
        self
    }

    pub fn with_material(mut self, material: impl Material + Send + Sync + 'static) -> Self {
        self.material = Some(Box::new(material));
        self
    }

    pub fn with_child(mut self, child: SceneNode) -> Self {
        self.children.push(child);
        self
    }
}