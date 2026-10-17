//! Prepare one temporary project while retaining cleanup ownership.
use std::path::{Path, PathBuf};
use tracing::info;
/// Create an isolated Rust project and copy its PRD after the initial commit.
///
/// # Arguments
/// * `prd_path` - Original PRD file to copy.
/// * `project` - Rust package name and temporary directory prefix.
/// * `feature` - Feature description used for dependency selection.
/// * `stories` - Story title and description pairs, in their original order.
/// # Returns
/// Temporary directory owner and the copied PRD path; retain the owner during execution.
/// # Errors
/// Returns filesystem or process-launch errors, and unsuccessful Rust or Git initialization.
/// # Examples
/// ```no_run
/// let (owner, prd) = codetether_benchmark::workspace::scaffold_workspace(
///     std::path::Path::new("prd.json"), "sample", "CLI", std::iter::empty())?;
/// assert!(prd.starts_with(owner.path()));
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn scaffold_workspace<'a>(
    prd_path: &Path,
    project: &str,
    feature: &str,
    stories: impl Iterator<Item = (&'a str, &'a str)>,
) -> anyhow::Result<(tempfile::TempDir, PathBuf)> {
    let tmp = tempfile::Builder::new()
        .prefix(&format!("bench-{}-", project))
        .tempdir()?;
    let workspace = tmp.path();

    info!(
        "Scaffolding benchmark workspace at {:?} for PRD '{}'",
        workspace, feature
    );

    super::initialize_rust::initialize_rust(workspace, project)?;
    // Add dependencies based on PRD requirements
    let feature_lower = feature.to_lowercase();
    let desc_text: String = stories
        .map(|(title, description)| format!("{} {}", title, description))
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();

    super::dependencies::add_dependencies(workspace, &feature_lower, &desc_text);
    super::initialize_git::initialize_git(workspace)?;
    // Copy the benchmark PRD into the workspace as prd.json
    let dest_prd = workspace.join("prd.json");
    std::fs::copy(prd_path, &dest_prd)?;

    Ok((tmp, dest_prd))
}
