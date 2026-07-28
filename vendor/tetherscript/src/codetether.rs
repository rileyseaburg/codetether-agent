//! CodeTether compatibility helpers.
//!
//! CodeTether Agent already has a signed plugin manifest shape. This module
//! lets a TetherScript plugin describe itself with `fn plugin()` and emits that
//! manifest without making CodeTether understand TetherScript internals.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{json, Value as Json};
use sha2::{Digest, Sha256};

use crate::plugin::{TetherScriptAuthority, PluginHost};
use crate::value::{ResultValue, Value};

#[derive(Debug)]
pub enum ManifestError {
    Io { path: PathBuf, message: String },
    Plugin(String),
    Metadata(String),
}

pub fn manifest_for_file(path: impl AsRef<Path>) -> Result<Json, ManifestError> {
    let path = path.as_ref();
    let source = fs::read_to_string(path).map_err(|e| ManifestError::Io {
        path: path.to_path_buf(),
        message: e.to_string(),
    })?;
    manifest_for_source(path.display().to_string(), &source)
}

pub fn manifest_for_source(name: impl Into<String>, source: &str) -> Result<Json, ManifestError> {
    let mut host = PluginHost::new();
    host.grant("tetherscript", TetherScriptAuthority::new());

    let mut plugin = host
        .load_source(name.into(), source)
        .map_err(|e| ManifestError::Plugin(e.to_string()))?;
    let call = plugin
        .metadata()
        .map_err(|e| ManifestError::Plugin(e.to_string()))?
        .ok_or_else(|| ManifestError::Metadata("plugin must define `fn plugin()`".into()))?;
    let metadata = unwrap_result(call.value)?;
    let map = match metadata {
        Value::Map(map) => map,
        other => {
            return Err(ManifestError::Metadata(format!(
                "plugin() must return a map, got {}",
                other.type_name()
            )));
        }
    };
    let map = map.borrow();

    let id = string_field(&map, "id")
        .or_else(|| string_field(&map, "name"))
        .ok_or_else(|| ManifestError::Metadata("plugin metadata needs `id` or `name`".into()))?;
    let name = string_field(&map, "name").unwrap_or_else(|| id.clone());
    let version = string_field(&map, "version").unwrap_or_else(|| "0.1.0".into());
    let signed_by = string_field(&map, "signed_by").unwrap_or_else(|| "unsigned".into());
    let signature = string_field(&map, "signature").unwrap_or_default();
    let capabilities = string_list_field(&map, "capabilities")?;
    let timeout_secs = int_field(&map, "timeout_secs").unwrap_or(5).max(0) as u64;

    Ok(json!({
        "id": id,
        "name": name,
        "version": version,
        "content_hash": format!("sha256:{}", sha256_hex(source.as_bytes())),
        "signed_by": signed_by,
        "signature": signature,
        "capabilities": capabilities,
        "timeout_secs": timeout_secs,
    }))
}

fn unwrap_result(value: Value) -> Result<Value, ManifestError> {
    match value {
        Value::Result(result) => match result.as_ref() {
            ResultValue::Ok(value) => Ok(value.clone()),
            ResultValue::Err(message) => Err(ManifestError::Metadata(format!(
                "plugin() returned Err({message:?})"
            ))),
        },
        value => Ok(value),
    }
}

fn string_field(map: &std::collections::HashMap<String, Value>, field: &str) -> Option<String> {
    match map.get(field) {
        Some(Value::Str(value)) => Some((**value).clone()),
        _ => None,
    }
}

fn int_field(map: &std::collections::HashMap<String, Value>, field: &str) -> Option<i64> {
    match map.get(field) {
        Some(Value::Int(value)) => Some(*value),
        _ => None,
    }
}

fn string_list_field(
    map: &std::collections::HashMap<String, Value>,
    field: &str,
) -> Result<Vec<String>, ManifestError> {
    let Some(value) = map.get(field) else {
        return Ok(Vec::new());
    };
    let Value::List(items) = value else {
        return Err(ManifestError::Metadata(format!(
            "`{field}` must be a list of strings"
        )));
    };
    items
        .borrow()
        .iter()
        .map(|item| match item {
            Value::Str(value) => Ok((**value).clone()),
            other => Err(ManifestError::Metadata(format!(
                "`{field}` entries must be strings, got {}",
                other.type_name()
            ))),
        })
        .collect()
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut out = String::with_capacity(digest.len() * 2);
    for byte in digest {
        out.push(hex_digit(byte >> 4));
        out.push(hex_digit(byte & 0x0f));
    }
    out
}

fn hex_digit(n: u8) -> char {
    match n {
        0..=9 => (b'0' + n) as char,
        10..=15 => (b'a' + (n - 10)) as char,
        _ => unreachable!(),
    }
}

impl std::fmt::Display for ManifestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ManifestError::Io { path, message } => write!(f, "{}: {}", path.display(), message),
            ManifestError::Plugin(message) | ManifestError::Metadata(message) => {
                write!(f, "{message}")
            }
        }
    }
}

impl std::error::Error for ManifestError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emits_codetether_manifest_from_tetherscript_metadata() {
        let source = r#"
fn plugin() {
    let m = map()
    m.id = "tetherscript.example"
    m.name = "TetherScript Example"
    m.version = "1.2.3"
    m.capabilities = ["fs.read", "tetherscript.diagnose"]
    m.timeout_secs = 9
    return m
}
"#;
        let manifest = manifest_for_source("example.tether", source).unwrap();
        assert_eq!(manifest["id"], "tetherscript.example");
        assert_eq!(manifest["name"], "TetherScript Example");
        assert_eq!(manifest["version"], "1.2.3");
        assert_eq!(manifest["capabilities"][0], "fs.read");
        assert_eq!(manifest["timeout_secs"], 9);
        assert!(manifest["content_hash"]
            .as_str()
            .unwrap()
            .starts_with("sha256:"));
        assert_eq!(manifest["signed_by"], "unsigned");
    }
}
