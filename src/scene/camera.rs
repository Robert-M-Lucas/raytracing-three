use crate::{FSize, scene::scene_node::SceneNode};

#[derive(Clone, Debug)]
pub struct Camera {
    pub fov: FSize, // radians
}

impl Camera {
    pub fn new(fov_deg: FSize) -> Self {
        Camera { fov: (fov_deg / 180 as FSize) * std::f64::consts::PI as FSize }
    }

    pub fn scene_node(self) -> SceneNode {
        SceneNode::new_camera(self)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self { fov: 19.7 as FSize }
    }
}
