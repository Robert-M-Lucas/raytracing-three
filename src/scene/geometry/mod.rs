use crate::{FSize, scene::geometry::mesh::Mesh};

pub mod mesh;

/// - `Sphere`: radius `radius`, centred at origin (Tungsten scenes use radius 1 + node scale)
/// - `Quad`: unit square in the XZ plane, centred, normal +Y
/// - `Disk`: unit-radius disk in the XZ plane, normal +Y
/// - `Cube`: axis-aligned cube spanning [-0.5, 0.5] on each axis
#[derive(Clone, Debug)]
pub enum Geometry {
    Sphere(Sphere),
    Quad,
    Disk,
    Cube,
    Mesh(Mesh),
}

#[derive(Clone, Debug)]
pub struct Sphere {
    pub radius: FSize,
}
