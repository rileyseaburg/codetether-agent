//! One workspace-relative display name and its source path.
use std::path::PathBuf;

/// One ranked candidate: its workspace-relative display string and full path.
/// # Examples
/// ```
/// let entry = codetether_fuzzy_finder::FuzzyEntry {
///     display: "src/main.rs".into(), path: "/workspace/src/main.rs".into(),
/// };
/// assert_eq!(entry.display, "src/main.rs");
/// ```
#[derive(Debug, Clone)]
pub struct FuzzyEntry {
    /// Workspace-relative label shown in results.
    pub display: String,
    /// Path opened when this result is confirmed.
    pub path: PathBuf,
}
