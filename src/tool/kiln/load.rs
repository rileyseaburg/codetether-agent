use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use super::input::KilnPluginInput;

pub async fn source(input: &KilnPluginInput, root: &Path) -> Result<(String, String)> {
    if let Some(source) = input.source.as_deref().filter(|source| !source.is_empty()) {
        return Ok(("inline.kiln".to_string(), source.to_string()));
    }

    let path = input.path.as_deref().context("missing Kiln plugin path")?;
    let path = resolve_plugin_path(root, path).await?;
    let source = tokio::fs::read_to_string(&path)
        .await
        .with_context(|| format!("Failed to read Kiln plugin file: {}", path.display()))?;
    Ok((path.display().to_string(), source))
}

async fn resolve_plugin_path(root: &Path, raw_path: &str) -> Result<PathBuf> {
    let root = tokio::fs::canonicalize(root)
        .await
        .with_context(|| format!("Failed to resolve Kiln plugin root: {}", root.display()))?;
    let raw = Path::new(raw_path);
    let candidate = if raw.is_absolute() {
        raw.to_path_buf()
    } else {
        root.join(raw)
    };
    let candidate = tokio::fs::canonicalize(&candidate).await.with_context(|| {
        format!(
            "Failed to resolve Kiln plugin file: {}",
            candidate.display()
        )
    })?;
    validate_plugin_path(&candidate, &root)?;
    Ok(candidate)
}

fn validate_plugin_path(candidate: &Path, root: &Path) -> Result<()> {
    if !candidate.starts_with(root) {
        anyhow::bail!(
            "Kiln plugin path '{}' escapes workspace root '{}'",
            candidate.display(),
            root.display()
        );
    }
    if candidate.extension().and_then(|ext| ext.to_str()) != Some("kl") {
        anyhow::bail!(
            "Kiln plugin path '{}' must use the .kl extension",
            candidate.display()
        );
    }
    Ok(())
}
