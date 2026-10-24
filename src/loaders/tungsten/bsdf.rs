use serde_json::Value;

use super::{
    LoadError,
    assets::{Assets, ColourSpace},
    json,
};
use crate::{
    maths::vector::V3,
    scene::{
        material::{Conductor, Distribution, Material, MaterialKind, Roughness},
        texture::Texture,
    },
};

pub fn parse_bsdf(v: &Value, assets: &mut Assets, ctx: &str) -> Result<Material, LoadError> {
    let ty =
        json::get_str(v, "type").ok_or_else(|| LoadError::invalid(ctx, "bsdf has no \"type\""))?;

    let bump = v
        .get("bump")
        .map(|b| assets.texture(b, ColourSpace::Linear, ctx))
        .transpose()?;

    let kind = match ty {
        "null" => MaterialKind::Null,
        "lambert" => MaterialKind::Lambert {
            albedo: albedo(v, assets, ctx)?,
        },
        "mirror" => MaterialKind::Mirror {
            albedo: albedo(v, assets, ctx)?,
        },
        "dielectric" | "rough_dielectric" => MaterialKind::Dielectric {
            albedo: albedo(v, assets, ctx)?,
            ior: json::get_f(v, "ior", 1.5)?,
            roughness: (ty == "rough_dielectric")
                .then(|| roughness(v))
                .transpose()?,
        },
        "conductor" | "rough_conductor" => MaterialKind::Conductor {
            albedo: albedo(v, assets, ctx)?,
            conductor: conductor(v, ctx)?,
            roughness: (ty == "rough_conductor")
                .then(|| roughness(v))
                .transpose()?,
        },
        "plastic" | "rough_plastic" => MaterialKind::Plastic {
            albedo: albedo(v, assets, ctx)?,
            ior: json::get_f(v, "ior", 1.5)?,
            thickness: json::get_f(v, "thickness", 1.0)?,
            sigma_a: json::get_vec3(v, "sigma_a", V3::ZERO, ctx)?,
            roughness: (ty == "rough_plastic").then(|| roughness(v)).transpose()?,
        },
        "thinsheet" => MaterialKind::ThinSheet {
            albedo: albedo(v, assets, ctx)?,
            ior: json::get_f(v, "ior", 1.5)?,
            thickness: json::get_f(v, "thickness", 0.5)?,
            sigma_a: json::get_vec3(v, "sigma_a", V3::ZERO, ctx)?,
            enable_interference: json::get_bool(v, "enable_interference", false),
        },
        "transparency" => {
            let base = v
                .get("base")
                .ok_or_else(|| LoadError::invalid(ctx, "transparency has no \"base\""))?;
            let alpha = v
                .get("alpha")
                .ok_or_else(|| LoadError::invalid(ctx, "transparency has no \"alpha\""))?;
            MaterialKind::Transparency {
                base: Box::new(parse_bsdf(base, assets, ctx)?),
                alpha: assets.texture(alpha, ColourSpace::Linear, ctx)?,
            }
        }
        other => {
            eprintln!("warning: {ctx}: unsupported bsdf \"{other}\", using lambert");
            MaterialKind::Lambert {
                albedo: albedo(v, assets, ctx)?,
            }
        }
    };

    Ok(Material { kind, bump })
}

fn albedo(v: &Value, assets: &mut Assets, ctx: &str) -> Result<Texture, LoadError> {
    match v.get("albedo") {
        Some(a) => assets.texture(a, ColourSpace::Srgb, ctx),
        None => Ok(Texture::grey(1.0)),
    }
}

fn roughness(v: &Value) -> Result<Roughness, LoadError> {
    let distribution = match json::get_str(v, "distribution").map(str::to_ascii_lowercase) {
        Some(d) if d == "beckmann" => Distribution::Beckmann,
        Some(d) if d == "phong" => Distribution::Phong,
        _ => Distribution::Ggx,
    };
    Ok(Roughness {
        distribution,
        roughness: json::get_f(v, "roughness", 0.1)?,
    })
}

fn conductor(v: &Value, ctx: &str) -> Result<Conductor, LoadError> {
    if let (Some(eta), Some(k)) = (v.get("eta"), v.get("k")) {
        return Ok(Conductor::Custom {
            eta: json::vec3(eta, ctx)?,
            k: json::vec3(k, ctx)?,
        });
    }
    Ok(Conductor::Named(
        json::get_str(v, "material").unwrap_or("Cu").to_owned(),
    ))
}
