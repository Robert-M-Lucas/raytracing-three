use serde_json::Value;

use super::LoadError;
use crate::{FSize, maths::vector::V3};

pub fn num(v: &Value, ctx: &str) -> Result<FSize, LoadError> {
    v.as_f64()
        .map(|x| x as FSize)
        .ok_or_else(|| LoadError::invalid(ctx, "expected a number"))
}

/// A number is splatted to all three channels; otherwise `[x, y, z]` is required.
pub fn vec3(v: &Value, ctx: &str) -> Result<V3, LoadError> {
    if let Some(x) = v.as_f64() {
        let x = x as FSize;
        return Ok(V3::new_v3(x, x, x));
    }
    if let Some(a) = v.as_array()
        && let [x, y, z] = a.as_slice()
    {
        return Ok(V3::new_v3(num(x, ctx)?, num(y, ctx)?, num(z, ctx)?));
    }
    Err(LoadError::invalid(ctx, "expected a number or [x, y, z]"))
}

pub fn get_f(obj: &Value, key: &str, default: FSize) -> Result<FSize, LoadError> {
    match obj.get(key) {
        Some(v) => num(v, key),
        None => Ok(default),
    }
}

pub fn get_vec3(obj: &Value, key: &str, default: V3, ctx: &str) -> Result<V3, LoadError> {
    match obj.get(key) {
        Some(v) => vec3(v, &format!("{ctx}: {key}")),
        None => Ok(default),
    }
}

pub fn get_bool(obj: &Value, key: &str, default: bool) -> bool {
    obj.get(key).and_then(Value::as_bool).unwrap_or(default)
}

pub fn get_str<'a>(obj: &'a Value, key: &str) -> Option<&'a str> {
    obj.get(key).and_then(Value::as_str)
}
