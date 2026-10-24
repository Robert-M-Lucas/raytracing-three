//! Tungsten (`scene.json`) loader. Accepts a scene directory, a `scene.json`, or a `.zip`.
//!
//! Everything (textures, meshes) is read into memory during `load_scene`, so a zip's
//! temporary extraction directory is deleted before the function returns.

mod assets;
mod bsdf;
mod json;
mod wo3;

use std::{
    collections::HashMap,
    fs::{self, File},
    io::BufReader,
    path::{Path, PathBuf},
    sync::Arc,
};

use serde_json::Value;
use tempfile::TempDir;

use crate::{
    FSize,
    maths::{matrix::M4, vector::V3},
    scene::{
        camera::Camera,
        geometry::{Geometry, Sphere, mesh::Mesh},
        material::Material,
        scene_node::SceneNode,
        scene_object::SceneObject,
    },
};
use assets::{Assets, ColourSpace};

#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("zip error: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("failed to decode image {path}: {source}")]
    Image {
        path: PathBuf,
        source: image::ImageError,
    },
    #[error("no scene.json found in {0}")]
    SceneNotFound(PathBuf),
    #[error("asset path escapes the scene directory: {0}")]
    PathEscape(String),
    #[error("{context}: {message}")]
    Invalid { context: String, message: String },
}

impl LoadError {
    pub fn invalid(context: impl Into<String>, message: impl Into<String>) -> Self {
        LoadError::Invalid {
            context: context.into(),
            message: message.into(),
        }
    }
}

pub fn load_tungsten_scene(path: impl AsRef<Path>) -> Result<SceneNode, LoadError> {
    let path = path.as_ref();
    println!("Loading scene {path:?}");

    let is_zip = path
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("zip"));

    if is_zip {
        println!("Extracing zip to temp dir");
        let tmp = TempDir::new()?;
        zip::ZipArchive::new(BufReader::new(File::open(path)?))?.extract(tmp.path())?;
        let scene_json = find_scene_json(tmp.path())?;
        load_scene_json(&scene_json) // `tmp` dropped (and deleted) after everything is in memory
    } else if path.is_dir() {
        load_scene_json(&find_scene_json(path)?)
    } else {
        load_scene_json(path)
    }
}

fn find_scene_json(dir: &Path) -> Result<PathBuf, LoadError> {
    let mut level = vec![dir.to_path_buf()];
    for _ in 0..4 {
        let mut next = Vec::new();
        let mut candidates = Vec::new();
        for d in &level {
            for entry in fs::read_dir(d)? {
                let p = entry?.path();
                if p.is_dir() {
                    if p.file_name().is_some_and(|n| n != "__MACOSX") {
                        next.push(p);
                    }
                } else if p.file_name().is_some_and(|n| n == "scene.json") {
                    return Ok(p);
                } else if p
                    .extension()
                    .is_some_and(|e| e.eq_ignore_ascii_case("json"))
                {
                    candidates.push(p);
                }
            }
        }
        if candidates.len() == 1 {
            return Ok(candidates.remove(0));
        }
        level = next;
    }
    Err(LoadError::SceneNotFound(dir.to_path_buf()))
}

fn load_scene_json(scene_json: &Path) -> Result<SceneNode, LoadError> {
    println!("Loading scene json");
    let root_dir = scene_json.parent().unwrap_or(Path::new("."));
    let doc: Value = serde_json::from_str(&fs::read_to_string(scene_json)?)?;
    let mut assets = Assets::new(root_dir)?;

    // BSDFs (referenced by name from primitives).
    let mut bsdfs: HashMap<String, Material> = HashMap::new();
    if let Some(list) = doc.get("bsdfs").and_then(Value::as_array) {
        for (i, b) in list.iter().enumerate() {
            let name = json::get_str(b, "name")
                .map(str::to_owned)
                .unwrap_or_else(|| format!("bsdf#{i}"));
            let mat = bsdf::parse_bsdf(b, &mut assets, &format!("bsdf \"{name}\""))?;
            bsdfs.insert(name, mat);
        }
    }

    let mut root = SceneNode::new_empty();

    if let Some(cam) = doc.get("camera") {
        root.add_child(parse_camera(cam)?);
    }

    if let Some(prims) = doc.get("primitives").and_then(Value::as_array) {
        for (i, p) in prims.iter().enumerate() {
            if let Some(node) = parse_primitive(p, i, &mut assets, &bsdfs)? {
                root.add_child(node);
            }
        }
    }
    Ok(root)
}

/// Only the position and fov are used; orientation/look_at/tonemap/etc. are ignored.
fn parse_camera(c: &Value) -> Result<SceneNode, LoadError> {
    let fov = json::get_f(c, "fov", 60.0)?;
    let t = c.get("transform").unwrap_or(&Value::Null);
    let p = json::get_vec3(t, "position", V3::ZERO, "camera position")?;
    Ok(Camera::new(fov)
        .scene_node()
        .with_transform(M4::translation(p.x(), p.y(), p.z())))
}

fn parse_primitive(
    p: &Value,
    index: usize,
    assets: &mut Assets,
    bsdfs: &HashMap<String, Material>,
) -> Result<Option<SceneNode>, LoadError> {
    let name = json::get_str(p, "name").map(str::to_owned);
    let ctx = format!(
        "primitive #{index}{}",
        name.as_deref()
            .map(|n| format!(" \"{n}\""))
            .unwrap_or_default()
    );

    let ty =
        json::get_str(p, "type").ok_or_else(|| LoadError::invalid(&ctx, "missing \"type\""))?;
    let geometry = match ty {
        "mesh" => {
            let file = json::get_str(p, "file")
                .ok_or_else(|| LoadError::invalid(&ctx, "mesh has no \"file\""))?;
            let mut data = assets.mesh(file)?;
            if json::get_bool(p, "recompute_normals", false) {
                let mut copy = (*data).clone();
                copy.recompute_normals();
                data = Arc::new(copy);
            }
            Geometry::Mesh(Mesh {
                data,
                smooth: json::get_bool(p, "smooth", true),
                backface_culling: json::get_bool(p, "backface_culling", false),
            })
        }
        "sphere" => Geometry::Sphere(Sphere { radius: 1.0 }),
        "quad" => Geometry::Quad,
        "disk" => Geometry::Disk,
        "cube" => Geometry::Cube,
        other => {
            eprintln!("warning: {ctx}: unsupported primitive type \"{other}\", skipped");
            return Ok(None);
        }
    };

    let material = match p.get("bsdf") {
        Some(Value::String(n)) => bsdfs
            .get(n)
            .cloned()
            .ok_or_else(|| LoadError::invalid(&ctx, format!("unknown bsdf \"{n}\"")))?,
        Some(inline @ Value::Object(_)) => bsdf::parse_bsdf(inline, assets, &ctx)?,
        _ => Material::default(),
    };

    let mut object = SceneObject::new(geometry, material);
    object.name = name;
    if let Some(e) = p.get("emission") {
        object.emission = Some(assets.texture(e, ColourSpace::Srgb, &ctx)?);
    }
    if let Some(pw) = p.get("power") {
        object.power = Some(json::vec3(pw, &ctx)?);
    }

    let transform = parse_transform(p.get("transform"), &ctx)?;
    Ok(Some(object.scene_node().with_transform(transform)))
}

fn parse_transform(t: Option<&Value>, ctx: &str) -> Result<M4, LoadError> {
    let Some(t) = t else {
        return Ok(M4::identity());
    };
 
    if let Some(arr) = t.as_array() {
        if arr.len() != 16 {
            return Err(LoadError::invalid(ctx, "matrix transform needs 16 numbers"));
        }
        let mut d = [[0.0 as FSize; 4]; 4];
        for (i, x) in arr.iter().enumerate() {
            d[i / 4][i % 4] = json::num(x, ctx)?;
        }
        return Ok(M4::new_m(d));
    }
 
    let pos = json::get_vec3(t, "position", V3::ZERO, ctx)?;
    let scale = json::get_vec3(t, "scale", V3::ONE, ctx)?;
 
    let rotation = match t.get("look_at") {
        // `look_at` takes precedence over `rotation`, as in Tungsten.
        Some(target) => {
            let target = json::vec3(target, ctx)?;
            let up = json::get_vec3(t, "up", V3::UP, ctx)?;
            look_at_rotation(pos, target, up, ctx)?
        }
        None => {
            let rot = json::get_vec3(t, "rotation", V3::ZERO, ctx)?;
            let to_rad = std::f64::consts::PI as FSize / 180.0;
            M4::rotation_y(rot.y() * to_rad)
                .cross(&M4::rotation_x(rot.x() * to_rad))
                .cross(&M4::rotation_z(rot.z() * to_rad))
        }
    };
 
    Ok(M4::translation(pos.x(), pos.y(), pos.z())
        .cross(&rotation)
        .cross(&M4::scaling(scale.x(), scale.y(), scale.z())))
}

fn look_at_rotation(eye: V3, target: V3, up: V3, ctx: &str) -> Result<M4, LoadError> {
    type A = [FSize; 3];
    fn arr(v: V3) -> A {
        [v.x(), v.y(), v.z()]
    }
    fn cross(a: A, b: A) -> A {
        [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ]
    }
    fn normalise(a: A) -> Option<A> {
        let l = (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt();
        (l > 1e-8).then(|| [a[0] / l, a[1] / l, a[2] / l])
    }
 
    let (e, t, u) = (arr(eye), arr(target), arr(up));
    let f = normalise([t[0] - e[0], t[1] - e[1], t[2] - e[2]])
        .ok_or_else(|| LoadError::invalid(ctx, "look_at equals position"))?;
    let r = normalise(cross(u, f))
        .ok_or_else(|| LoadError::invalid(ctx, "up is zero or parallel to the view direction"))?;
    let y = cross(f, r);
 
    Ok(M4::new_m([
        [r[0], y[0], f[0], 0.0],
        [r[1], y[1], f[1], 0.0],
        [r[2], y[2], f[2], 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]))
}