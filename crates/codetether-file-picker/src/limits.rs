//! Frontend defaults for bounded browsing and file previews.
//!
//! These constants describe the native picker limits and page-navigation step.
/// Maximum entries retained by a picker listing.
pub const FILE_PICKER_MAX_ENTRIES: usize = 500;
/// Maximum file bytes loaded for a compact preview.
pub const FILE_PICKER_PREVIEW_MAX_BYTES: usize = 8 * 1024;
/// Maximum lines shown in a compact file preview.
pub const FILE_PICKER_PREVIEW_MAX_LINES: usize = 14;
/// Maximum child entries shown for a directory preview.
pub const FILE_PICKER_PREVIEW_DIR_ITEMS: usize = 10;
/// Number of entries traversed by page navigation.
pub const FILE_PICKER_PAGE_STEP: usize = 12;
