use crate::{
    FSize,
    maths::{lines::Line, matrix::M4},
    pipelines::shared::bvh::{BVHNode, Bvh, BvhSceneObjectRef},
    scene::scene_object::SceneObject,
};

pub struct BvhHitIterator<'a> {
    bvh: &'a Bvh,
    ray: Line,
    stack: Vec<usize>,
    current_leaf_refs: std::slice::Iter<'a, BvhSceneObjectRef>,
    t_min: FSize,
    t_max: FSize,
}

impl<'a> BvhHitIterator<'a> {
    pub fn new(bvh: &'a Bvh, ray: Line, t_min: FSize, t_max: FSize) -> Self {
        let mut stack = Vec::new();
        if !bvh.nodes.is_empty() {
            stack.push(bvh.nodes.len() - 1); // Push root node index
        }
        Self {
            bvh,
            ray,
            stack,
            current_leaf_refs: [].iter(),
            t_min,
            t_max,
        }
    }
}

impl<'a> Iterator for BvhHitIterator<'a> {
    type Item = (&'a (M4, SceneObject), u32);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Yield the next reference from the current leaf if available
            if let Some(r) = self.current_leaf_refs.next() {
                return Some((
                    &self.bvh.scene_objects[r.object_index as usize],
                    r.subobject_index,
                ));
            }

            // Pop the next node from the traversal stack; return None if empty
            let node_idx = self.stack.pop()?;

            match &self.bvh.nodes[node_idx] {
                BVHNode::Internal { aabb, left, right } => {
                    if aabb.intersect(&self.ray, self.t_min, self.t_max) {
                        // Push right child first so that the left child is processed first
                        self.stack.push(*right);
                        self.stack.push(*left);
                    }
                }
                BVHNode::Leaf { aabb, refs } => {
                    if aabb.intersect(&self.ray, self.t_min, self.t_max) {
                        self.current_leaf_refs = refs.iter();
                    }
                }
            }
        }
    }
}
