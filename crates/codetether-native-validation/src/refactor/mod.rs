//! Native validation-report binding for shared refactor rules.
use crate::ValidationReport;
use anyhow::Result;
use std::path::{Path, PathBuf};
mod hook;
mod plugin;
/// Evaluate configured refactor rules with the native optional plugin hook.
/// # Arguments
/// * root - Workspace root.
/// * paths - Files touched by the current task.
/// # Returns
/// An optional report using the common validation report format.
/// # Errors
/// Propagates shared refactor-rule and plugin failures.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let report = codetether_native_validation::refactor::evaluate(
///     std::path::Path::new("."), &[],
/// ).await?;
/// # Ok(()) }
/// ```
pub async fn evaluate(root: &Path, paths: &[PathBuf]) -> Result<Option<ValidationReport>> {
    Ok(
        codetether_refactor_guard::evaluate(root, paths, &hook::NativeHook)
            .await?
            .map(|report| ValidationReport {
                issue_count: report.issue_count,
                prompt: report.prompt,
            }),
    )
}
