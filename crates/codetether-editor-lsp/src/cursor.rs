//! Convert editor cursor coordinates to LSP request coordinates.
use codetether_editor::FileBuffer;
/// Resolve a file buffer and cursor to one-based LSP coordinates.
/// # Arguments
/// * `buffer` - Current editor buffer, if any.
/// # Returns
/// File path, one-based line and one-based column, or None.
/// # Examples
/// ```
/// assert!(codetether_editor_lsp::cursor_target(None).is_none());
/// ```
pub fn cursor_target(buffer: Option<&FileBuffer>) -> Option<(std::path::PathBuf, u32, u32)> {
    let buf = buffer?;
    let (line, col) = buf.cursor();
    Some((buf.path().to_path_buf(), line as u32 + 1, col as u32 + 1))
}
