//! Navigate a buffer to a server-provided source location.
use codetether_editor::FileBuffer;
use codetether_lsp_core::{LocationInfo, uri_to_path};
/// Open a definition target when necessary and move its buffer cursor.
/// # Arguments
/// * `buffer` - Active buffer slot.
/// * `scroll` - Vertical scroll, reset when opening another file.
/// * `hscroll` - Horizontal scroll, reset when opening another file.
/// * `loc` - Zero-based target location from the server.
/// # Returns
/// Navigation or file-open status; None if no buffer is available.
/// # Examples
/// ```no_run
/// # fn example(loc: &codetether_lsp_core::LocationInfo) {
/// let (mut buffer, mut scroll, mut hscroll) = (None, 0, 0);
/// let status = codetether_editor_lsp::navigate(&mut buffer, &mut scroll, &mut hscroll, loc);
/// # }
/// ```
pub fn navigate(
    buffer: &mut Option<FileBuffer>,
    scroll: &mut usize,
    hscroll: &mut usize,
    loc: &LocationInfo,
) -> Option<String> {
    let target = uri_to_path(&loc.uri);
    let line = loc.range.start.line as usize;
    let col = loc.range.start.character as usize;

    let same_file = buffer.as_ref().is_some_and(|b| b.path() == target);

    if !same_file {
        match FileBuffer::open(&target) {
            Ok(buf) => {
                *buffer = Some(buf);
                *scroll = 0;
                *hscroll = 0;
            }
            Err(e) => {
                return Some(format!("Cannot open {}: {e}", target.display()));
            }
        }
    }

    if let Some(buf) = buffer.as_mut() {
        buf.set_cursor(line, col);
        return Some(format!("→ {}:{}", target.display(), line + 1));
    }

    None
}
