use crate::tool::sandbox::SandboxPolicy;
use anyhow::{Context, Result, anyhow};
use std::path::{Path, PathBuf};

pub async fn validate_working_dir(policy: &SandboxPolicy, work_dir: &Path) -> Result<()> {
    if policy.allowed_paths.is_empty() {
        return Ok(());
    }
    let actual = canonical(work_dir)
        .await
        .with_context(|| format!("Failed to resolve sandbox cwd: {}", work_dir.display()))?;
    let allowed_paths = canonical_allowed_paths(&policy.allowed_paths).await?;
    if allowed_paths
        .iter()
        .any(|allowed| actual.starts_with(allowed))
    {
        return Ok(());
    }
    Err(anyhow!(
        "Sandbox denied working directory outside allowed paths: {}",
        actual.display()
    ))
}

async fn canonical_allowed_paths(paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let mut resolved = Vec::with_capacity(paths.len());
    for path in paths {
        resolved.push(canonical(path).await?);
    }
    Ok(resolved)
}

async fn canonical(path: &Path) -> Result<PathBuf> {
    tokio::fs::canonicalize(path)
        .await
        .with_context(|| format!("Failed to resolve sandbox path: {}", path.display()))
}
