use crate::tool::sandbox::SandboxPolicy;
use anyhow::{Context, Result, anyhow};
use std::path::{Path, PathBuf};

pub fn validate_working_dir(policy: &SandboxPolicy, work_dir: &Path) -> Result<()> {
    if policy.allowed_paths.is_empty() {
        return Ok(());
    }
    let actual = canonical(work_dir)
        .with_context(|| format!("Failed to resolve sandbox cwd: {}", work_dir.display()))?;
    for allowed in &policy.allowed_paths {
        if actual.starts_with(canonical(allowed)?) {
            return Ok(());
        }
    }
    Err(anyhow!(
        "Sandbox denied working directory outside allowed paths: {}",
        actual.display()
    ))
}

fn canonical(path: &Path) -> Result<PathBuf> {
    path.canonicalize()
        .with_context(|| format!("Failed to resolve sandbox path: {}", path.display()))
}
