pub mod camera;
pub mod geometry;
pub mod material;
pub mod scene_node;
pub mod scene_object;
pub mod texture;

use crate::maths::vector::V3;
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AABB {
    pub min: V3,
    pub max: V3,
}
