//! Native validation-report binding for shared refactor rules.
use super::validation::ValidationReport;
use anyhow::Result;
use std::path::{Path, PathBuf};
mod hook;
mod plugin;
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
