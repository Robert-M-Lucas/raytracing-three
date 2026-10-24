use std::sync::Arc;

use crate::{FSize, maths::vector::V3};

#[derive(Clone, Debug)]
pub struct Mesh {
    pub data: Arc<MeshData>,
    pub smooth: bool,
    pub backface_culling: bool,
}

#[derive(Clone, Debug, Default)]
pub struct MeshData {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<Triangle>,
}

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: V3,
    pub normal: V3,
    pub uv: (FSize, FSize),
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub indices: [u32; 3],
    /// Per-triangle material slot from the file (-1 / 0 usually means "none").
    pub material: i32,
}

impl MeshData {
    /// Area-weighted smooth vertex normals.
    pub fn recompute_normals(&mut self) {
        let mut acc = vec![[0.0 as FSize; 3]; self.vertices.len()];
        for t in &self.triangles {
            let [a, b, c] = t.indices.map(|i| i as usize);
            let (p0, p1, p2) = (
                self.vertices[a].position,
                self.vertices[b].position,
                self.vertices[c].position,
            );
            let (e1, e2) = (
                [p1.x() - p0.x(), p1.y() - p0.y(), p1.z() - p0.z()],
                [p2.x() - p0.x(), p2.y() - p0.y(), p2.z() - p0.z()],
            );
            let n = [
                e1[1] * e2[2] - e1[2] * e2[1],
                e1[2] * e2[0] - e1[0] * e2[2],
                e1[0] * e2[1] - e1[1] * e2[0],
            ];
            for i in [a, b, c] {
                for k in 0..3 {
                    acc[i][k] += n[k];
                }
            }
        }
        for (v, n) in self.vertices.iter_mut().zip(acc) {
            let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
            if len > 0.0 {
                v.normal = V3::new_v3(n[0] / len, n[1] / len, n[2] / len);
            }
        }
    }
}
