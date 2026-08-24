//! Bind and validate an active orchestration permit.

use super::{FIELD, map};
use anyhow::Result;
use serde_json::{Value, json};
use std::path::Path;

pub(crate) fn bind(args: &mut Value, cwd: &Path) -> Result<bool> {
    let cwd = cwd.canonicalize()?;
    let active = map()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some((workspace, (token, _))) = active.iter().find(|(root, _)| cwd.starts_with(root))
    else {
        return Ok(false);
    };
    let Some(values) = args.as_object_mut() else {
        return Ok(false);
    };
    values.insert(FIELD.into(), json!(token));
    values.insert("__ct_parent_workspace".into(), json!(workspace));
    Ok(true)
}

pub(crate) fn allows(tool: &str, args: &Value) -> bool {
    if tool != "bash" {
        return false;
    }
    let Some(token) = args.get(FIELD).and_then(Value::as_str) else {
        return false;
    };
    let Some(cwd) = args.get("cwd").and_then(Value::as_str) else {
        return false;
    };
    let Ok(cwd) = Path::new(cwd).canonicalize() else {
        return false;
    };
    map()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .iter()
        .any(|(root, (expected, _))| token == expected && cwd.starts_with(root))
}
