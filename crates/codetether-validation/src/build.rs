//! Validate existing touched files, stopping early for refactor violations.
use crate::{Host, ValidationReport, validation_scope};
use anyhow::Result;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

/// # Arguments
/// Receives workspace, touched files, baseline dirty files and native validation services.
/// # Returns
/// A continuation report when diagnostics remain, otherwise None.
/// # Errors
/// Propagates native refactor guard errors.
/// # Examples
/// ```no_run
/// # async fn example(host: &impl codetether_validation::Host) -> anyhow::Result<()> {
/// let files = std::collections::HashSet::new();
/// assert!(codetether_validation::build_validation_report(std::path::Path::new("."), &files, &files, host).await?.is_none());
/// # Ok(())
/// # }
/// ```
pub async fn build_validation_report<H: Host>(
    workspace_dir: &Path,
    touched_files: &HashSet<PathBuf>,
    baseline_git_dirty_files: &HashSet<PathBuf>,
    host: &H,
) -> Result<Option<ValidationReport>> {
    let candidate_files = validation_scope(touched_files, baseline_git_dirty_files);

    let mut existing_files: Vec<PathBuf> = candidate_files
        .into_iter()
        .filter(|path| path.is_file())
        .collect();
    existing_files.sort();
    existing_files.dedup();

    if existing_files.is_empty() {
        return Ok(None);
    }
    if let Some(report) = host.refactor(workspace_dir, &existing_files).await? {
        return Ok(Some(report));
    }

    let root_uri = format!("file://{}", workspace_dir.display());
    let manager = host.manager(root_uri).await;
    let (issues_by_file, issue_count) =
        super::collect::run(workspace_dir, existing_files, manager.as_ref(), host).await;
    Ok(super::render::report(issues_by_file, issue_count))
}
