//! Trusted workspace confinement for mux lifecycle operations.

use super::args::Args;
use anyhow::{Result, anyhow};
use serde_json::Value;

pub(super) fn bind(args: &mut Args, raw: &mut Value) -> Result<()> {
    if !matches!(args.action.as_str(), "start" | "roll") {
        return Ok(());
    }
    let parent = crate::tool::network_access::trusted_workspace(raw)
        .map(std::path::PathBuf::from)
        .unwrap_or(std::env::current_dir()?)
        .canonicalize()?;
    let requested = args.workspace.as_deref().unwrap_or(&parent);
    let requested = if requested.is_absolute() {
        requested.to_path_buf()
    } else {
        parent.join(requested)
    };
    let workspace = requested.canonicalize()?;
    if !workspace.starts_with(&parent) {
        return Err(anyhow!("mux workspace is outside trusted parent"));
    }
    args.workspace = Some(workspace.clone());
    raw.as_object_mut()
        .ok_or_else(|| anyhow!("mux arguments must be an object"))?
        .insert(
            "workspace".into(),
            workspace.display().to_string().into(),
        );
    Ok(())
}