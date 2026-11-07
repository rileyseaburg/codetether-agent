//! Install the hooks a repository declares, mirroring the Kubernetes Job bootstrap.
//!
//! Policy:
//! * `.githooks/pre-push` → `bash scripts/dev/install-pre-push-cache-gate.sh`,
//!   which must exist and owns every `.githooks/<name>` it manages. It runs
//!   first, after stale pre-commit-generated copies of those hooks are removed
//!   (see [`super::managed`]);
//! * `.pre-commit-config.yaml` → `pre-commit install --install-hooks`. When
//!   the cache gate is declared, only hook types it does not own are passed
//!   explicitly, so pre-commit never writes a hook the gate then refuses.
//!
//! Both installers are idempotent, so re-preparing a clone succeeds.
use super::installer::{run, timeout};
use super::managed::{owns, remove_superseded};
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
    install(repo_path, "pre-commit")
        .await
        .context("repository hooks could not be installed")
}

/// Install with an explicit `pre-commit` program, for tests.
#[cfg(test)]
pub(super) async fn install_with(repo_path: &Path, pre_commit: &str) -> Result<Vec<&'static str>> {
    install(repo_path, pre_commit)
        .await
        .context("repository hooks could not be installed")
}

async fn install(repo_path: &Path, pre_commit: &str) -> Result<Vec<&'static str>> {
    let limit = timeout();
    let mut installed = Vec::new();
    let gate = repo_path.join(PRE_PUSH_HOOK).is_file();
    if gate {
        if !repo_path.join(PRE_PUSH_INSTALLER).is_file() {
            bail!("{PRE_PUSH_HOOK} is declared but {PRE_PUSH_INSTALLER} is missing");
        }
        remove_superseded(repo_path).await?;
        run(repo_path, "bash", &[PRE_PUSH_INSTALLER], limit).await?;
        installed.push("pre-push");
    }
    if repo_path.join(PRE_COMMIT_CONFIG).is_file() {
        let args = pre_commit_args(repo_path, gate);
        run(repo_path, pre_commit, &args, limit).await?;
        installed.push("pre-commit");
    }
    Ok(installed)
}

/// Arguments for `pre-commit`, excluding hook types `.githooks/` owns.
fn pre_commit_args(repo_path: &Path, gate: bool) -> Vec<&'static str> {
    let mut args = vec!["install", "--install-hooks"];
    if gate {
        args.extend(["--hook-type", "pre-commit"]);
        if declares_post_commit(repo_path) && !owns(repo_path, "post-commit") {
            args.extend(["--hook-type", "post-commit"]);
        }
    }
    args
}

fn declares_post_commit(repo_path: &Path) -> bool {
    std::fs::read_to_string(repo_path.join(PRE_COMMIT_CONFIG))
        .is_ok_and(|text| text.contains("post-commit"))
}
