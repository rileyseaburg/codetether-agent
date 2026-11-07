//! Install the hooks a repository declares, mirroring the Kubernetes Job bootstrap.
//!
//! Policy (identical to `scripts/harvester/job_bootstrap.py::install_hooks`):
//! * `.pre-commit-config.yaml` → `pre-commit install --install-hooks`, which
//!   honours the repository's `default_install_hook_types`;
//! * `.githooks/pre-push` → `bash scripts/dev/install-pre-push-cache-gate.sh`,
//!   which must exist.
//!
//! Both installers are idempotent, so re-preparing a clone succeeds.
use super::installer::{run, timeout};
use anyhow::{Context, Result, bail};
use std::path::Path;

const PRE_COMMIT_CONFIG: &str = ".pre-commit-config.yaml";
const PRE_PUSH_HOOK: &str = ".githooks/pre-push";
const PRE_PUSH_INSTALLER: &str = "scripts/dev/install-pre-push-cache-gate.sh";

/// Install every hook `repo_path` declares, or fail without running a task.
///
/// Returns the installer names that ran; an empty list means nothing is
/// declared. Installation is not verified here; callers check the result
/// with [`super::require_installed`].
///
/// # Errors
/// Returns `repository hooks could not be installed (...)` when `pre-commit`
/// is missing from `PATH`, the pre-push installer script is missing, or an
/// installer exits nonzero or times out.
/// # Examples
/// ```rust,no_run
/// # async fn example() -> anyhow::Result<()> {
/// use codetether_worktree::repository::install_declared_hooks;
/// let installed = install_declared_hooks(std::path::Path::new("repo")).await?;
/// println!("installed: {installed:?}");
/// # Ok(()) }
/// ```
pub async fn install_declared_hooks(repo_path: &Path) -> Result<Vec<&'static str>> {
    install(repo_path)
        .await
        .context("repository hooks could not be installed")
}

async fn install(repo_path: &Path) -> Result<Vec<&'static str>> {
    let limit = timeout();
    let mut installed = Vec::new();
    if repo_path.join(PRE_COMMIT_CONFIG).is_file() {
        run(
            repo_path,
            "pre-commit",
            &["install", "--install-hooks"],
            limit,
        )
        .await?;
        installed.push("pre-commit");
    }
    if repo_path.join(PRE_PUSH_HOOK).is_file() {
        if !repo_path.join(PRE_PUSH_INSTALLER).is_file() {
            bail!("{PRE_PUSH_HOOK} is declared but {PRE_PUSH_INSTALLER} is missing");
        }
        run(repo_path, "bash", &[PRE_PUSH_INSTALLER], limit).await?;
        installed.push("pre-push");
    }
    Ok(installed)
}
