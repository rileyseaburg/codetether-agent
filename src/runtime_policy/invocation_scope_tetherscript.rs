//! Loaded-source identity for TetherScript approval resources.

use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

pub(super) fn bind(args: &mut Value, network: bool) {
    crate::tool::network_access::bind(args, network);
    if let Some(map) = args.as_object_mut() {
        map.remove("__ct_tetherscript_source_sha256");
    }
    let Some(source) = source(args) else {
        return;
    };
    if let Some(map) = args.as_object_mut() {
        map.insert(
            "__ct_tetherscript_source_sha256".into(),
            json!(hex::encode(Sha256::digest(source.as_bytes()))),
        );
    }
}

fn source(args: &Value) -> Option<String> {
    if let Some(source) = args.get("source").and_then(Value::as_str) {
        return Some(source.to_string());
    }
    let root = crate::tool::network_access::trusted_workspace(args)
        .map(PathBuf::from)?
        .canonicalize()
        .ok()?;
    let requested = PathBuf::from(args.get("path")?.as_str()?);
    let requested = if requested.is_absolute() {
        requested
    } else {
        root.join(requested)
    };
    let path = requested.canonicalize().ok()?;
    path.starts_with(&root)
        .then(|| std::fs::read_to_string(path).ok())?
}
