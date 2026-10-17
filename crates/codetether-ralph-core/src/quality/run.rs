//! Execute configured quality commands in order and report each result.
use crate::{QualityChecks, verification::ShellExecutor};
use std::path::Path;
use tracing::debug;
/// Run quality checks, stopping at the first failed command.
///
/// # Arguments
/// * `dir` - Workspace to search for a Rust project root.
/// * `checks` - Optional commands in typecheck, lint, test and build order.
/// * `executor` - Host shell execution, including policy enforcement.
/// * `on_result` - Callback invoked for each executed check before failure logging.
/// # Returns
/// True if every configured check passed, including when none is configured.
/// # Errors
/// Returns host execution errors with the check name attached.
/// # Examples
/// ```no_run
/// # async fn example(executor: &dyn codetether_ralph_core::verification::ShellExecutor) -> anyhow::Result<()> {
/// let checks = codetether_ralph_core::QualityChecks::default();
/// let passed = codetether_ralph_core::quality::run_quality_gates(std::path::Path::new("."), &checks, executor, |_, _| {}).await?;
/// assert!(passed);
/// # Ok(()) }
/// ```
pub async fn run_quality_gates(
    dir: &Path,
    checks: &QualityChecks,
    executor: &dyn ShellExecutor,
    mut on_result: impl FnMut(&str, bool),
) -> anyhow::Result<bool> {
    // Find the Cargo root (where Cargo.toml lives)
    let cargo_root = super::find_root::find_cargo_root(dir);
    debug!(
        worktree_dir = %dir.display(),
        cargo_root = %cargo_root.display(),
        "Running quality gates"
    );

    let mut all_passed = true;

    for (name, cmd) in [
        ("typecheck", &checks.typecheck),
        ("lint", &checks.lint),
        ("test", &checks.test),
        ("build", &checks.build),
    ] {
        if let Some(command) = cmd {
            debug!("Running {} check in {:?}: {}", name, cargo_root, command);
            let output = executor
                .execute(&cargo_root, command)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to run quality check '{}': {}", name, e))?;

            let passed = output.success;
            on_result(name, passed);

            if !passed {
                super::failure::report_failure(name, &cargo_root, &output.output);
                all_passed = false;
                break; // Short-circuit: stop at first failure to save time
            }
        }
    }

    Ok(all_passed)
}
