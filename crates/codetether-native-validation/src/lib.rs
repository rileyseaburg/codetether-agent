//! Native services for the shared touched-file validation pipeline.
//!
//! [NativeHost] runs refactor checks, obtains the native LSP manager and gathers
//! policy-gated ESLint diagnostics. [build_validation_report] supplies these
//! services to the shared validation sequence while preserving dirty baselines.
use anyhow::Result;
pub use codetether_validation::{ValidationReport, capture_git_dirty_files, track_touched_files};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};
mod host;
pub use host::NativeHost;
mod eslint;
pub mod refactor;
/// Build native diagnostics for touched files while respecting dirty baselines.
/// # Arguments
/// * workspace_dir - Workspace containing the touched files.
/// * touched_files - Paths attributed to the current task.
/// * baseline_git_dirty_files - Files already dirty before the task started.
/// # Returns
/// An optional validation report using the shared validation rules.
/// # Errors
/// Propagates errors from shared refactor or diagnostic processing.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let report = codetether_native_validation::build_validation_report(
///     std::path::Path::new("."), &Default::default(), &Default::default(),
/// ).await?;
/// # Ok(()) }
/// ```
pub async fn build_validation_report(
    workspace_dir: &Path,
    touched_files: &HashSet<PathBuf>,
    baseline_git_dirty_files: &HashSet<PathBuf>,
) -> Result<Option<ValidationReport>> {
    codetether_validation::build_validation_report(
        workspace_dir,
        touched_files,
        baseline_git_dirty_files,
        &host::NativeHost,
    )
    .await
}
