// Layout (little endian):
//   u64 vertex_count
//   vertex_count * { f32 pos[3]; f32 normal[3]; f32 uv[2] }          (32 bytes)
//   u64 triangle_count
//   triangle_count * { u32 v0, v1, v2; i32 material }                 (16 bytes)
//
// The triangle stride is inferred from the remaining file size, so a variant without
// the per-triangle material field (12 bytes) is also accepted.

use std::{fs, path::Path};

use super::LoadError;
use crate::{
    FSize,
    maths::vector::V3,
    scene::geometry::mesh::{MeshData, Triangle, Vertex},
};

const VERTEX_BYTES: usize = 32;

fn u64_at(b: &[u8], o: usize) -> Option<u64> {
    let end = o.checked_add(8)?;
    Some(u64::from_le_bytes(b.get(o..end)?.try_into().ok()?))
}

fn f32_at(b: &[u8], o: usize) -> f32 {
    f32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}

fn u32_at(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}

pub fn read_wo3(path: &Path) -> Result<MeshData, LoadError> {
    let bytes = fs::read(path)?;
    let bad = |m: &str| LoadError::invalid(path.display().to_string(), m);

    let nv = u64_at(&bytes, 0).ok_or_else(|| bad("truncated header"))? as usize;
    let tri_count_offset = nv
        .checked_mul(VERTEX_BYTES)
        .and_then(|n| n.checked_add(8))
        .ok_or_else(|| bad("vertex count overflow"))?;
    let nt = u64_at(&bytes, tri_count_offset).ok_or_else(|| bad("truncated vertex data"))? as usize;
    let tri_start = tri_count_offset + 8;
    let remaining = bytes.len() - tri_start;

    let stride = match (nt.checked_mul(16), nt.checked_mul(12)) {
        (Some(n), _) if n == remaining => 16,
        (_, Some(n)) if n == remaining => 12,
        _ => return Err(bad("file size does not match header counts")),
    };

    let vertices = (0..nv)
        .map(|i| {
            let o = 8 + i * VERTEX_BYTES;
            let f = |k: usize| f32_at(&bytes, o + k * 4) as FSize;
            Vertex {
                position: V3::new_v3(f(0), f(1), f(2)),
                normal: V3::new_v3(f(3), f(4), f(5)),
                uv: (f(6), f(7)),
            }
        })
        .collect();

    let mut triangles = Vec::with_capacity(nt);
    for i in 0..nt {
        let o = tri_start + i * stride;
        let indices = [
            u32_at(&bytes, o),
            u32_at(&bytes, o + 4),
            u32_at(&bytes, o + 8),
        ];
        if indices.iter().any(|&idx| idx as usize >= nv) {
            return Err(bad("triangle index out of range"));
        }
        let material = if stride == 16 {
            u32_at(&bytes, o + 12) as i32
        } else {
            0
        };
        triangles.push(Triangle { indices, material });
    }

    Ok(MeshData {
        vertices,
        triangles,
    })
}
