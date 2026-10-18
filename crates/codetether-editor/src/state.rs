//! Active editor document and the viewport offsets associated with it.
use crate::FileBuffer;

/// Mutable document and viewport state for an embedded editor.
///
/// Frontends keep their modal navigation, approval sessions and language-server
/// state separately. An empty editor starts at the top-left of its viewport.
///
/// # Examples
/// ```
/// use codetether_editor::{EditorState, FileBuffer};
/// let mut editor = EditorState::default();
/// assert!(editor.buffer.is_none());
/// assert_eq!((editor.scroll, editor.hscroll), (0, 0));
/// editor.buffer = Some(FileBuffer::draft("notes.txt", "hello"));
/// assert_eq!(editor.buffer.as_ref().unwrap().text(), "hello");
/// ```
#[derive(Default)]
pub struct EditorState {
    /// Active document; absent when no buffer is open.
    pub buffer: Option<FileBuffer>,
    /// First visible document line.
    pub scroll: usize,
    /// First visible document column.
    pub hscroll: usize,
}
