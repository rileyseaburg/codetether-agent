//! load_progress file operation.
use std::path::Path;
/// Read progress, treating read failures as empty content.
///
/// # Arguments
/// * `working_dir` - Workspace containing the progress file.
/// * `progress_path` - Configured relative or absolute progress path.
/// # Returns
/// Progress text, or empty text when reading fails.
/// # Examples
/// ```no_run
/// let text = codetether_ralph_core::progress::load_progress(std::path::Path::new("."), "progress.txt")?;
/// let _ = text;
/// # Ok::<(), anyhow::Error>(())
/// ```
/// # Errors
/// The current implementation returns Ok even if the file cannot be read.

/// Load progress file
pub fn load_progress(working_dir: &Path, progress_path: &str) -> anyhow::Result<String> {
    let path = working_dir.join(progress_path);
    Ok(std::fs::read_to_string(path).unwrap_or_default())
}
