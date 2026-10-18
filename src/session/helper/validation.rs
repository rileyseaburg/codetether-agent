//! Native services for the shared touched-file validation pipeline.
use anyhow::Result;
pub use codetether_validation::{ValidationReport, capture_git_dirty_files, track_touched_files};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};
#[path = "validation_host.rs"]
mod host;
#[path = "validation_eslint.rs"]
mod validation_eslint;
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
