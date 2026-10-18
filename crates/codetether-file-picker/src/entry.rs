//! File picker FilePickerEntry data.

use std::path::PathBuf;

/// One directory listing entry.
/// # Examples
/// ```
/// let entry = codetether_file_picker::FilePickerEntry { path: "note.txt".into(), is_dir: false, name: "note.txt".into() };
/// assert!(!entry.is_dir);
/// ```
#[derive(Debug, Clone)]
pub struct FilePickerEntry {
    /// Filesystem path.
    pub path: PathBuf,
    /// Whether this entry represents a directory.
    pub is_dir: bool,
    /// Display label.
    pub name: String,
}
