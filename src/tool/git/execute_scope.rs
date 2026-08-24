//! Workspace binding for structured Git invocations.

use anyhow::{Result, anyhow};
use serde_json::Value;

pub(super) fn bind_cwd(args: &mut Value) -> Result<()> {
    let root = crate::tool::network_access::trusted_workspace(args)
        .map(std::path::PathBuf::from)
        .unwrap_or(std::env::current_dir()?)
        .canonicalize()?;
    let requested = args.get("cwd").and_then(Value::as_str).unwrap_or(".");
    let requested = std::path::Path::new(requested);
    let cwd = if requested.is_absolute() {
        requested.into()
    } else {
        root.join(requested)
    };
    let cwd = cwd.canonicalize()?;
    if !cwd.starts_with(&root) {
        return Err(anyhow!("git cwd is outside workspace: {}", cwd.display()));
    }
    args.as_object_mut()
        .ok_or_else(|| anyhow!("git arguments must be an object"))?
        .insert("cwd".into(), cwd.display().to_string().into());
    Ok(())
}
