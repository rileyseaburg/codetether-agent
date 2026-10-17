//! append_progress file operation.
use super::load_progress;
use crate::ProgressEntry;
use std::path::Path;
/// Append a formatted iteration summary to the progress file.
///
/// # Arguments
/// * `working_dir` - Workspace containing the progress file.
/// * `progress_path` - Configured relative or absolute progress path.
/// * `entry` - Iteration metadata.
/// * `response` - Agent response text.
/// # Returns
/// Unit after writing the updated progress file.
/// # Examples
/// ```no_run
/// # fn example(entry: &codetether_ralph_core::ProgressEntry) -> anyhow::Result<()> {
/// codetether_ralph_core::progress::append_progress(std::path::Path::new("."), "progress.txt", entry, "summary")?;
/// # Ok(()) }
/// ```
/// # Errors
/// Returns the file-writing error.

/// Append to progress file
pub fn append_progress(
    working_dir: &Path,
    progress_path: &str,
    entry: &ProgressEntry,
    response: &str,
) -> anyhow::Result<()> {
    let path = working_dir.join(progress_path);
    let mut content = load_progress(working_dir, progress_path).unwrap_or_default();

    content.push_str(&format!(
        "\n---\n\n## Iteration {} - {} ({})\n\n**Status:** {}\n\n### Summary\n{}\n",
        entry.iteration, entry.story_id, entry.timestamp, entry.status, response
    ));

    std::fs::write(path, content)?;
    Ok(())
}
