use std::collections::VecDeque;

use crate::maths::matrix::M4;
use crate::scene::camera::Camera;
use crate::scene::scene_object::SceneObject;

#[derive(Clone, Debug)]
enum SceneNodeType {
    Empty,
    Object(SceneObject),
    Camera(Camera),
}

#[derive(Clone, Debug)]
pub struct SceneNode {
    pub name: Option<String>,
    pub transform: M4,
    node_type: SceneNodeType,
    pub children: Vec<SceneNode>,
}

impl SceneNode {
    pub fn new_empty() -> Self {
        Self {
            name: None,
            transform: M4::IDENTITY,
            node_type: SceneNodeType::Empty,
            children: Vec::new(),
        }
    }

    pub fn new_object(scene_object: SceneObject) -> Self {
        Self {
            name: None,
            transform: M4::IDENTITY,
            node_type: SceneNodeType::Object(scene_object),
            children: Vec::new(),
        }
    }

    pub fn new_camera(camera: Camera) -> Self {
        Self {
            name: None,
            transform: M4::IDENTITY,
            node_type: SceneNodeType::Camera(camera),
            children: Vec::new(),
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_transform(mut self, transform: M4) -> Self {
        self.transform = transform;
        self
    }

    pub fn transform_with(&mut self, transform: M4) {
        self.transform = self.transform.cross(&transform);
    }

    pub fn with_child(mut self, child: SceneNode) -> Self {
        self.children.push(child);
        self
    }

    pub fn add_child(&mut self, child: SceneNode) {
        self.children.push(child);
    }

    pub fn camera(&self) -> Option<&Camera> {
        match &self.node_type {
            SceneNodeType::Empty => None,
            SceneNodeType::Object(_) => None,
            SceneNodeType::Camera(camera) => Some(camera),
        }
    }

    pub fn scene_object(&self) -> Option<&SceneObject> {
        match &self.node_type {
            SceneNodeType::Empty => None,
            SceneNodeType::Object(scene_object) => Some(scene_object),
            SceneNodeType::Camera(_) => None,
        }
    }

    pub fn camera_mut(&mut self) -> Option<&mut Camera> {
        match &mut self.node_type {
            SceneNodeType::Empty => None,
            SceneNodeType::Object(_) => None,
            SceneNodeType::Camera(camera) => Some(camera),
        }
    }

    pub fn scene_object_mut(&mut self) -> Option<&mut SceneObject> {
        match &mut self.node_type {
            SceneNodeType::Empty => None,
            SceneNodeType::Object(scene_object) => Some(scene_object),
            SceneNodeType::Camera(_) => None,
        }
    }

    pub fn find_first_camera(&self) -> Option<&SceneNode> {
        let mut queue = VecDeque::from([self]);
        while let Some(node) = queue.pop_front() {
            if node.camera().is_some() {
                return Some(node);
            }
            queue.extend(node.children.iter());
        }
        None
    }

    pub fn find_first_camera_mut(&mut self) -> Option<&mut SceneNode> {
        let mut queue = VecDeque::from([self]);
        while let Some(node) = queue.pop_front() {
            if node.camera().is_some() {
                return Some(node);
            }
            queue.extend(node.children.iter_mut());
        }
        None
    }
}
