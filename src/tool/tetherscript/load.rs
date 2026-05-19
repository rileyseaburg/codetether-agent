mod resolve;
mod validate;

use std::path::Path;

use anyhow::{Context, Result};

use super::input::TetherScriptPluginInput;

pub async fn source(input: &TetherScriptPluginInput, root: &Path) -> Result<(String, String)> {
    if let Some(source) = inline_source(input) {
        return Ok(("inline.tether".to_string(), source));
    }
    file_source(input, root).await
}

fn inline_source(input: &TetherScriptPluginInput) -> Option<String> {
    input
        .source
        .as_deref()
        .filter(|source| !source.is_empty())
        .map(str::to_string)
}

async fn file_source(input: &TetherScriptPluginInput, root: &Path) -> Result<(String, String)> {
    let path = input
        .path
        .as_deref()
        .context("missing TetherScript plugin path")?;
    let path = resolve::plugin_path(root, path).await?;
    let source = tokio::fs::read_to_string(&path).await.with_context(|| {
        format!(
            "Failed to read TetherScript plugin file: {}",
            path.display()
        )
    })?;
    Ok((path.display().to_string(), source))
}
