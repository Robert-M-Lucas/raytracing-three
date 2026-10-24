use crate::{FSize, maths::vector::V3, scene::texture::Texture};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Distribution {
    Beckmann,
    Phong,
    Ggx,
}

#[derive(Clone, Copy, Debug)]
pub struct Roughness {
    pub distribution: Distribution,
    pub roughness: FSize,
}

#[derive(Clone, Debug)]
pub enum Conductor {
    /// Named measured material, e.g. "Al", "Cu", "Au".
    Named(String),
    Custom {
        eta: V3,
        k: V3,
    },
}

#[derive(Clone, Debug)]
pub enum MaterialKind {
    Null,
    Lambert {
        albedo: Texture,
    },
    Mirror {
        albedo: Texture,
    },
    Dielectric {
        albedo: Texture,
        ior: FSize,
        roughness: Option<Roughness>,
    },
    Conductor {
        albedo: Texture,
        conductor: Conductor,
        roughness: Option<Roughness>,
    },
    Plastic {
        albedo: Texture,
        ior: FSize,
        thickness: FSize,
        sigma_a: V3,
        roughness: Option<Roughness>,
    },
    ThinSheet {
        albedo: Texture,
        ior: FSize,
        thickness: FSize,
        sigma_a: V3,
        enable_interference: bool,
    },
    /// `base` blended with transparency using `alpha` (x channel).
    Transparency {
        base: Box<Material>,
        alpha: Texture,
    },
}

#[derive(Clone, Debug)]
pub struct Material {
    pub kind: MaterialKind,
    pub bump: Option<Texture>,
}

impl Material {
    pub fn new(kind: MaterialKind) -> Self {
        Self { kind, bump: None }
    }

    pub fn lambert(albedo: Texture) -> Self {
        Self::new(MaterialKind::Lambert { albedo })
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::lambert(Texture::grey(0.5))
    }
}
