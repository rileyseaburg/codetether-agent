//! Trusted workspace binding for the scoped Git adapter.

use anyhow::{Result, anyhow};
use serde_json::{Value, json};
use std::path::Path;

pub(super) fn bind(args: &mut Value, root: &Path) -> Result<()> {
    let network_allowed = crate::tool::network_access::trusted_value(args).unwrap_or(false);
    let fields = args
        .as_object_mut()
        .ok_or_else(|| anyhow!("git arguments must be an object"))?;
    fields.insert("cwd".into(), Value::String(root.display().to_string()));
    fields.insert(
        "__ct_parent_workspace".into(), Value::String(root.display().to_string()),
    );
    fields
        .entry("__ct_session_id")
        .or_insert_with(|| json!("swarm-scoped-git"));
    crate::tool::network_access::bind_trusted(args, network_allowed);
    Ok(())
}