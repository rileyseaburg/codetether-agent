//! File picker FilePickerMode data.

/// Whether the picker shows a directory or full file preview.
/// # Examples
/// ```
/// use codetether_file_picker::FilePickerMode;
/// let text = match FilePickerMode::Browse { FilePickerMode::Browse => "browser", FilePickerMode::Viewer => "viewer" };
/// assert_eq!(text, "browser");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilePickerMode {
    /// Directory listing with side preview.
    Browse,
    /// Full file preview with scrolling.
    Viewer,
}
