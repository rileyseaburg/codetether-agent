use anyhow::Result;
use serde_json::Value;
use std::path::Path;

mod access;
mod clean;
mod path;

pub fn field(args: &mut Value, spec: &str, root: &Path) -> Result<()> {
    let Some(value) = access::get_mut_path(args, spec) else {
        return Ok(());
    };
    let Some(raw) = value.as_str() else {
        return Ok(());
    };
    if spec == "pattern" && raw.starts_with('*') {
        return Ok(());
    }
    *value = Value::String(path::normalize(raw, root)?.display().to_string());
    Ok(())
}
