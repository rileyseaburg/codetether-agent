//! Run quality commands and return feedback for the repair loop.
use crate::{QualityChecks, verification::ShellExecutor};
use std::path::Path;
/// Run quality checks, retaining the first failure as repair feedback.
///
/// # Arguments
/// * `story_working_dir` - Workspace searched for its Rust project root.
/// * `quality_checks` - Commands in typecheck, lint, test and build order.
/// * `executor` - Host shell executor with policy enforcement.
/// # Returns
/// Whether all configured commands passed and the first failure's feedback.
/// Execution errors become feedback instead of propagating from this function.
/// # Examples
/// ```no_run
/// # async fn example(executor: &dyn codetether_ralph_core::verification::ShellExecutor) {
/// let (ok, feedback) = codetether_ralph_core::quality::run_for_repair(std::path::Path::new("."), &codetether_ralph_core::QualityChecks::default(), executor).await;
/// assert!(ok && feedback.is_empty());
/// # }
/// ```
pub async fn run_for_repair(
    story_working_dir: &Path,
    quality_checks: &QualityChecks,
    executor: &dyn ShellExecutor,
) -> (bool, String) {
    let cargo_root = super::find_root::find_cargo_root(story_working_dir);
    let checks = quality_checks;
    let mut all_ok = true;
    let mut error_output = String::new();

    for (name, cmd) in [
        ("typecheck", &checks.typecheck),
        ("lint", &checks.lint),
        ("test", &checks.test),
        ("build", &checks.build),
    ] {
        if let Some(command) = cmd {
            let cmd_output = executor.execute(&cargo_root, command).await;
            match cmd_output {
                Ok(o) if o.success => {}
                Ok(o) => {
                    error_output = super::repair_error::format_error(name, &o.output);
                    all_ok = false;
                    break;
                }
                Err(e) => {
                    error_output = format!("Quality check `{name}` could not run: {e}");
                    all_ok = false;
                    break;
                }
            }
        }
    }

    (all_ok, error_output)
}
