use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use serde_json::Value;

use super::{LoadError, json, wo3};
use crate::{
    FSize,
    maths::vector::V3,
    scene::{
        geometry::mesh::MeshData,
        texture::{ImageData, Texture},
    },
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ColourSpace {
    /// 8-bit colour images: decoded to linear.
    Srgb,
    /// Data maps (bump, alpha): values used as stored.
    Linear,
}

/// Resolves paths relative to the scene directory (refusing escapes) and
/// de-duplicates images/meshes so identical files share one `Arc`.
pub struct Assets {
    root: PathBuf,
    images: HashMap<(PathBuf, ColourSpace), Arc<ImageData>>,
    meshes: HashMap<PathBuf, Arc<MeshData>>,
}

impl Assets {
    pub fn new(root: &Path) -> Result<Self, LoadError> {
        Ok(Self {
            root: root.canonicalize()?,
            images: HashMap::new(),
            meshes: HashMap::new(),
        })
    }

    fn resolve(&self, rel: &str) -> Result<PathBuf, LoadError> {
        let joined = self.root.join(rel.replace('\\', "/"));
        let path = joined
            .canonicalize()
            .map_err(|e| LoadError::invalid(rel, format!("cannot open asset: {e}")))?;
        if !path.starts_with(&self.root) {
            return Err(LoadError::PathEscape(rel.to_owned()));
        }
        Ok(path)
    }

    pub fn mesh(&mut self, rel: &str) -> Result<Arc<MeshData>, LoadError> {
        let path = self.resolve(rel)?;
        if let Some(m) = self.meshes.get(&path) {
            return Ok(m.clone());
        }
        let mesh = Arc::new(wo3::read_wo3(&path)?);
        self.meshes.insert(path, mesh.clone());
        Ok(mesh)
    }

    pub fn image(&mut self, rel: &str, space: ColourSpace) -> Result<Texture, LoadError> {
        let path = self.resolve(rel)?;
        let key = (path.clone(), space);
        if let Some(img) = self.images.get(&key) {
            return Ok(Texture::Image(img.clone()));
        }
        let img = Arc::new(load_image(&path, space)?);
        self.images.insert(key, img.clone());
        Ok(Texture::Image(img))
    }

    /// Number / `[r,g,b]` -> solid colour, string (or `{ "file": ... }`) -> image.
    pub fn texture(
        &mut self,
        v: &Value,
        space: ColourSpace,
        ctx: &str,
    ) -> Result<Texture, LoadError> {
        match v {
            Value::String(s) => self.image(s, space),
            Value::Object(_) => match json::get_str(v, "file") {
                Some(f) => self.image(f, space),
                None => Err(LoadError::invalid(ctx, "unsupported procedural texture")),
            },
            _ => Ok(Texture::Solid(json::vec3(v, ctx)?)),
        }
    }
}

fn srgb_to_linear(x: f32) -> f32 {
    if x <= 0.04045 {
        x / 12.92
    } else {
        ((x + 0.055) / 1.055).powf(2.4)
    }
}

fn load_image(path: &Path, space: ColourSpace) -> Result<ImageData, LoadError> {
    let img = image::open(path).map_err(|source| LoadError::Image {
        path: path.to_path_buf(),
        source,
    })?;
    let is_hdr = path
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| matches!(e.to_ascii_lowercase().as_str(), "hdr" | "exr"));
    let decode = space == ColourSpace::Srgb && !is_hdr;

    let rgb = img.into_rgb32f();
    let (width, height) = rgb.dimensions();
    if width == 0 || height == 0 {
        return Err(LoadError::invalid(
            path.display().to_string(),
            "empty image",
        ));
    }
    let conv = |x: f32| -> FSize { (if decode { srgb_to_linear(x) } else { x }) as FSize };
    let pixels = rgb
        .pixels()
        .map(|p| V3::new_v3(conv(p.0[0]), conv(p.0[1]), conv(p.0[2])))
        .collect();

    Ok(ImageData {
        source: path.to_path_buf(),
        width,
        height,
        pixels,
    })
}
