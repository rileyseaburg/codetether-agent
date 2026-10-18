//! File picker FilePickerState data.
use crate::{FilePickerEntry, FilePickerMode, FilePreview};

use std::path::PathBuf;

/// Directory browser and read-only viewer state.
/// # Examples
/// ```
/// let state = codetether_file_picker::FilePickerState::default();
/// assert!(!state.active);
/// ```
#[derive(Debug, Clone)]
pub struct FilePickerState {
    /// Whether the picker is active.
    pub active: bool,
    /// Workspace root used for attachments.
    pub workspace_dir: PathBuf,
    /// Currently browsed directory.
    pub dir: PathBuf,
    /// Filtered visible entries.
    pub entries: Vec<FilePickerEntry>,
    /// Unfiltered listing for `dir`, cached so filtering does not re-read disk.
    pub all_entries: Vec<FilePickerEntry>,
    /// Selected visible-entry index.
    pub selected: usize,
    /// Current directory-name filter.
    pub filter: String,
    /// Browser or full-file viewer mode.
    pub mode: FilePickerMode,
    /// Cached preview for the selected entry.
    pub preview: Option<FilePreview>,
    /// First visible line in the full preview.
    pub preview_scroll: usize,
}
