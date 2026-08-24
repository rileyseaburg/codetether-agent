//! Trusted workspace and PRD-content binding for Ralph invocations.

#[path = "ralph_scope_path.rs"]
mod path;

use anyhow::{Result, anyhow};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

const HASH: &str = "__ct_ralph_prd_sha256";

pub(crate) fn bind(args: &mut Value) -> Result<PathBuf> {
    let root = crate::tool::network_access::trusted_workspace(args)
        .map(PathBuf::from)
        .unwrap_or(std::env::current_dir()?)
        .canonicalize()?;
    let requested = args
        .get("prd_path")
        .and_then(Value::as_str)
        .unwrap_or("prd.json");
    let path = path::resolve(&root, Path::new(requested))?;
    let values = args
        .as_object_mut()
        .ok_or_else(|| anyhow!("ralph arguments must be an object"))?;
    values.insert(
        "__ct_parent_workspace".into(),
        json!(root.display().to_string()),
    );
    values.insert("prd_path".into(), json!(path.display().to_string()));
    values.remove(HASH);
    if path.is_file() {
        let bytes = std::fs::read(&path)?;
        values.insert(HASH.into(), json!(hex::encode(Sha256::digest(bytes))));
    }
    Ok(root)
}
