//! Install and validate repository-declared hooks using a host-resolved active directory.
use anyhow::{Result, bail};
use std::path::{Path, PathBuf};
mod declared;
mod install;
#[cfg(test)]
mod install_order_tests;
#[cfg(test)]
mod install_tests;
mod installer;
mod managed;
mod runnable;
use declared::required_hooks;
pub use install::install_declared_hooks;
use runnable::executable;

/// Fail unless every hook the repository declares is installed and runnable.
pub async fn require_installed<F, Fut>(repo_path: &Path, hooks_dir: F) -> Result<()>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<PathBuf>>,
{
    let required = required_hooks(repo_path);
    if required.is_empty() {
        return Ok(());
    }
    let dir = hooks_dir().await?;
    let missing: Vec<&str> = required
        .into_iter()
        .filter(|name| !executable(&dir.join(name)))
        .collect();
    if !missing.is_empty() {
        bail!(
            "repository hooks are not installed ({}); refusing to commit or \
             push without them",
            missing.join(", ")
        );
    }
    Ok(())
}
