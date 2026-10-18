//! File picker FilePreview data.

use std::path::PathBuf;

/// Bounded preview content and format metadata.
/// # Examples
/// ```
/// let preview = codetether_file_picker::FilePreview { path: "note.txt".into(), lines: vec![], truncated: false, binary: false };
/// assert!(!preview.binary);
/// ```
#[derive(Debug, Clone)]
pub struct FilePreview {
    /// Filesystem path.
    pub path: PathBuf,
    /// Preview text lines.
    pub lines: Vec<String>,
    /// Whether the preview reached a byte or line limit.
    pub truncated: bool,
    /// Whether NUL bytes identified binary content.
    pub binary: bool,
}
