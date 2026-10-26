use std::fmt::Debug;

use crate::{
    FSize,
    maths::{lines::Line, matrix::M4, vector::V3},
    pipelines::shared::bvh::{aabb::AABB, hit_iterator::BVHHitIterator},
    scene::{
        geometry::{Geometry, Sphere},
        scene_node::SceneNode,
        scene_object::SceneObject,
    },
};

mod aabb;
pub mod hit_iterator;

const BVH_NODE_MIN_PRIM: usize = 4;

#[derive(Clone, Copy, Debug)]
struct BVHSceneObjectRef {
    object_index: u32,    // which scene object
    subobject_index: u32, // i.e. triangle index for a mesh, 0 for others
}

#[derive(Clone, Debug)]
enum BVHNode {
    Internal {
        aabb: AABB,
        left: usize,
        right: usize,
    },
    Leaf {
        aabb: AABB,
        refs: Vec<BVHSceneObjectRef>,
    },
}

struct BVHPrimitiveInfo {
    primitive_index: usize,
    object_ref: BVHSceneObjectRef,
    aabb: AABB,
    centroid: V3,
}

#[derive(Debug)]
pub struct BVH {
    scene_objects: Vec<(M4, SceneObject)>,
    nodes: Vec<BVHNode>, // root is last!
}

impl BVH {
    pub fn from_scene(scene: SceneNode) -> BVH {
        let mut scene_objects = Vec::new();
        Self::collect_scene_nodes(M4::IDENTITY, scene, &mut scene_objects);

        let mut primitives = Vec::new();

        for (object_index, (transform, object)) in scene_objects.iter().enumerate() {
            let obj_idx = object_index as u32;
            match &object.geometry {
                Geometry::Sphere(Sphere { radius }) => {
                    let local_aabb = AABB::new(
                        V3::new_v3(-*radius, -*radius, -*radius),
                        V3::new_v3(*radius, *radius, *radius),
                    );
                    let world_aabb = local_aabb.transform(transform);
                    primitives.push(BVHPrimitiveInfo {
                        primitive_index: primitives.len(),
                        object_ref: BVHSceneObjectRef {
                            object_index: obj_idx,
                            subobject_index: 0,
                        },
                        aabb: world_aabb,
                        centroid: world_aabb.centre(),
                    });
                }
                Geometry::Quad => {
                    let local_aabb =
                        AABB::new(V3::new_v3(-0.5, 0.0, -0.5), V3::new_v3(0.5, 0.0, 0.5));
                    let world_aabb = local_aabb.transform(transform);
                    primitives.push(BVHPrimitiveInfo {
                        primitive_index: primitives.len(),
                        object_ref: BVHSceneObjectRef {
                            object_index: obj_idx,
                            subobject_index: 0,
                        },
                        aabb: world_aabb,
                        centroid: world_aabb.centre(),
                    });
                }
                Geometry::Disk => {
                    let local_aabb =
                        AABB::new(V3::new_v3(-1.0, 0.0, -1.0), V3::new_v3(1.0, 0.0, 1.0));
                    let world_aabb = local_aabb.transform(transform);
                    primitives.push(BVHPrimitiveInfo {
                        primitive_index: primitives.len(),
                        object_ref: BVHSceneObjectRef {
                            object_index: obj_idx,
                            subobject_index: 0,
                        },
                        aabb: world_aabb,
                        centroid: world_aabb.centre(),
                    });
                }
                Geometry::Cube => {
                    let local_aabb =
                        AABB::new(V3::new_v3(-0.5, -0.5, -0.5), V3::new_v3(0.5, 0.5, 0.5));
                    let world_aabb = local_aabb.transform(transform);
                    primitives.push(BVHPrimitiveInfo {
                        primitive_index: primitives.len(),
                        object_ref: BVHSceneObjectRef {
                            object_index: obj_idx,
                            subobject_index: 0,
                        },
                        aabb: world_aabb,
                        centroid: world_aabb.centre(),
                    });
                }
                Geometry::Mesh(mesh) => {
                    for (sub_idx, tri) in mesh.data.triangles.iter().enumerate() {
                        let [i0, i1, i2] = tri.indices.map(|i| i as usize);
                        let p0 = mesh.data.vertices[i0].position;
                        let p1 = mesh.data.vertices[i1].position;
                        let p2 = mesh.data.vertices[i2].position;

                        // Transform triangle vertices to world space
                        let tp0 = transform.transform_v3(&p0);
                        let tp1 = transform.transform_v3(&p1);
                        let tp2 = transform.transform_v3(&p2);

                        let min = V3::new_v3(
                            tp0.x().min(tp1.x()).min(tp2.x()),
                            tp0.y().min(tp1.y()).min(tp2.y()),
                            tp0.z().min(tp1.z()).min(tp2.z()),
                        );
                        let max = V3::new_v3(
                            tp0.x().max(tp1.x()).max(tp2.x()),
                            tp0.y().max(tp1.y()).max(tp2.y()),
                            tp0.z().max(tp1.z()).max(tp2.z()),
                        );
                        let world_aabb = AABB::new(min, max);

                        primitives.push(BVHPrimitiveInfo {
                            primitive_index: primitives.len(),
                            object_ref: BVHSceneObjectRef {
                                object_index: obj_idx,
                                subobject_index: sub_idx as u32,
                            },
                            aabb: world_aabb,
                            centroid: world_aabb.centre(),
                        });
                    }
                }
            }
        }

        let mut nodes = Vec::new();
        let primitives_len = primitives.len();
        Self::build_recursive(&mut primitives, 0, primitives_len, &mut nodes);

        BVH {
            scene_objects,
            nodes,
        }
    }

    fn build_recursive(
        primitives: &mut [BVHPrimitiveInfo],
        start: usize,
        end: usize,
        nodes: &mut Vec<BVHNode>,
    ) -> usize {
        let n_primitives = end - start;
        let mut bounds = primitives[start].aabb;
        for i in (start + 1)..end {
            bounds = bounds.union(&primitives[i].aabb);
        }

        if n_primitives <= BVH_NODE_MIN_PRIM {
            let refs: Vec<BVHSceneObjectRef> = primitives[start..end]
                .iter()
                .map(|p| p.object_ref)
                .collect();
            let node_idx = nodes.len();
            nodes.push(BVHNode::Leaf { aabb: bounds, refs });
            return node_idx;
        }

        // Compute centroid bounds for SAH
        let mut centroid_bounds = AABB::new(primitives[start].centroid, primitives[start].centroid);
        for i in (start + 1)..end {
            centroid_bounds.min = V3::new_v3(
                centroid_bounds.min.x().min(primitives[i].centroid.x()),
                centroid_bounds.min.y().min(primitives[i].centroid.y()),
                centroid_bounds.min.z().min(primitives[i].centroid.z()),
            );
            centroid_bounds.max = V3::new_v3(
                centroid_bounds.max.x().max(primitives[i].centroid.x()),
                centroid_bounds.max.y().max(primitives[i].centroid.y()),
                centroid_bounds.max.z().max(primitives[i].centroid.z()),
            );
        }

        let extent = centroid_bounds.max - centroid_bounds.min;
        let axis = if extent.x() > extent.y() && extent.x() > extent.z() {
            0
        } else if extent.y() > extent.z() {
            1
        } else {
            2
        };

        if centroid_bounds.min.data[axis][0] == centroid_bounds.max.data[axis][0] {
            let refs: Vec<BVHSceneObjectRef> = primitives[start..end]
                .iter()
                .map(|p| p.object_ref)
                .collect();
            let node_idx = nodes.len();
            nodes.push(BVHNode::Leaf { aabb: bounds, refs });
            return node_idx;
        }

        // SAH Binning
        const NUM_BINS: usize = 16;
        #[derive(Clone, Copy)]
        struct Bin {
            bounds: AABB,
            primitive_count: usize,
        }

        let mut bins = [Bin {
            bounds: AABB::new(V3::new_v3(0.0, 0.0, 0.0), V3::new_v3(0.0, 0.0, 0.0)),
            primitive_count: 0,
        }; NUM_BINS];
        let mut bin_initialized = [false; NUM_BINS];

        let min_c = centroid_bounds.min.data[axis][0];
        let max_c = centroid_bounds.max.data[axis][0];
        let denom = max_c - min_c;

        for p in &primitives[start..end] {
            let c_val = p.centroid.data[axis][0];
            let mut b_idx = ((NUM_BINS as FSize) * (c_val - min_c) / denom) as usize;
            if b_idx >= NUM_BINS {
                b_idx = NUM_BINS - 1;
            }
            if bin_initialized[b_idx] {
                bins[b_idx].bounds = bins[b_idx].bounds.union(&p.aabb);
                bins[b_idx].primitive_count += 1;
            } else {
                bins[b_idx].bounds = p.aabb;
                bins[b_idx].primitive_count = 1;
                bin_initialized[b_idx] = true;
            }
        }

        // Evaluate split costs
        let mut min_cost = FSize::INFINITY;
        let mut best_split = 0;

        for i in 0..(NUM_BINS - 1) {
            let mut left_count = 0;
            let mut left_bounds = AABB::new(V3::new_v3(0.0, 0.0, 0.0), V3::new_v3(0.0, 0.0, 0.0));
            let mut left_init = false;

            for b in bins.iter().take(i + 1) {
                if b.primitive_count > 0 {
                    left_count += b.primitive_count;
                    if left_init {
                        left_bounds = left_bounds.union(&b.bounds);
                    } else {
                        left_bounds = b.bounds;
                        left_init = true;
                    }
                }
            }

            let mut right_count = 0;
            let mut right_bounds = AABB::new(V3::new_v3(0.0, 0.0, 0.0), V3::new_v3(0.0, 0.0, 0.0));
            let mut right_init = false;

            for b in bins.iter().skip(i + 1).take(NUM_BINS - (i + 1)) {
                if b.primitive_count > 0 {
                    right_count += b.primitive_count;
                    if right_init {
                        right_bounds = right_bounds.union(&b.bounds);
                    } else {
                        right_bounds = b.bounds;
                        right_init = true;
                    }
                }
            }

            if left_count == 0 || right_count == 0 {
                continue;
            }

            let cost = (left_count as FSize) * left_bounds.surface_area()
                + (right_count as FSize) * right_bounds.surface_area();

            if cost < min_cost {
                min_cost = cost;
                best_split = i;
            }
        }

        // Partition primitives based on best split bin
        let mid = {
            let pivot = min_c + (best_split + 1) as FSize * (denom / NUM_BINS as FSize);
            let mut i = start;
            let mut j = end - 1;
            loop {
                while i <= j && primitives[i].centroid.data[axis][0] < pivot {
                    i += 1;
                }
                while i <= j && primitives[j].centroid.data[axis][0] >= pivot {
                    if j == 0 {
                        break;
                    }
                    j -= 1;
                }
                if i >= j {
                    break;
                }
                primitives.swap(i, j);
                i += 1;
                j = j.saturating_sub(1);
            }
            i
        };

        if mid == start || mid == end {
            let mid_fallback = start + n_primitives / 2;
            let left_idx = Self::build_recursive(primitives, start, mid_fallback, nodes);
            let right_idx = Self::build_recursive(primitives, mid_fallback, end, nodes);
            let node_idx = nodes.len();
            nodes.push(BVHNode::Internal {
                aabb: bounds,
                left: left_idx,
                right: right_idx,
            });
            return node_idx;
        }

        let left_idx = Self::build_recursive(primitives, start, mid, nodes);
        let right_idx = Self::build_recursive(primitives, mid, end, nodes);

        let node_idx = nodes.len();
        nodes.push(BVHNode::Internal {
            aabb: bounds,
            left: left_idx,
            right: right_idx,
        });
        node_idx
    }

    fn collect_scene_nodes(transform: M4, node: SceneNode, nodes: &mut Vec<(M4, SceneObject)>) {
        let transform = transform.cross(&node.transform);

        if let Some(object) = node.scene_object() {
            nodes.push((transform, object.clone()));
        }

        for child in node.children {
            Self::collect_scene_nodes(transform, child, nodes);
        }
    }

    pub fn intersect<'a>(&'a self, ray: Line, t_min: FSize, t_max: FSize) -> BVHHitIterator<'a> {
        BVHHitIterator::new(self, ray, t_min, t_max)
    }
}