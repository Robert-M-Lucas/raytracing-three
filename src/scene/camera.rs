use crate::{
    FSize,
    scene::scene_node::SceneNode,
};

#[derive(Clone, Debug)]
pub struct Camera {
    pub fov: FSize,
}

impl Camera {
    pub fn new(fov: FSize) -> Self {
        Camera { fov }
    }

    pub fn scene_node(self) -> SceneNode {
        SceneNode::new_camera(self)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self { fov: 60 as FSize }
    }
}
