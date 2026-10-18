//! Borrowed editor fields with a frontend-selected view transition.
use super::{FinderView, View};
use crate::FileBuffer;
use codetether_fuzzy_finder::FuzzyFindState;

/// Editor and finder state borrowed from an interactive frontend.
///
/// The callback selects the editor view while preserving frontend-specific
/// navigation state. All other operations update only the supplied fields.
///
/// # Examples
/// ```rust
/// use codetether_editor::interactive::{EditorView, View};
/// let (mut finder, mut buffer) = (None, None);
/// let (mut vertical, mut horizontal) = (8, 2);
/// let mut status = String::new();
/// let mut redraw = false;
/// let mut view = EditorView {
///     finder: &mut finder, buffer: &mut buffer,
///     vertical_scroll: &mut vertical, horizontal_scroll: &mut horizontal,
///     status: &mut status, needs_redraw: &mut redraw, show_editor: || {},
/// };
/// view.reset_editor_scroll();
/// assert_eq!(*view.vertical_scroll, 0);
/// ```
pub struct EditorView<'a, F> {
    /// Active workspace finder, if open.
    pub finder: &'a mut Option<FuzzyFindState>,
    /// Active editor document, if open.
    pub buffer: &'a mut Option<FileBuffer>,
    /// First visible editor line.
    pub vertical_scroll: &'a mut usize,
    /// First visible editor column.
    pub horizontal_scroll: &'a mut usize,
    /// Frontend status message.
    pub status: &'a mut String,
    /// Frontend redraw request.
    pub needs_redraw: &'a mut bool,
    /// Transition to the editor view, including frontend navigation effects.
    pub show_editor: F,
}

impl<F: FnMut()> View for EditorView<'_, F> {
    fn finder(&mut self) -> &mut Option<FuzzyFindState> {
        self.finder
    }
    fn set_buffer(&mut self, buffer: FileBuffer) {
        *self.buffer = Some(buffer);
    }
    fn reset_editor_scroll(&mut self) {
        *self.vertical_scroll = 0;
        *self.horizontal_scroll = 0;
    }
    fn show_editor(&mut self) {
        (self.show_editor)();
    }
    fn set_status(&mut self, status: String) {
        *self.status = status;
    }
}

impl<F: FnMut()> FinderView for EditorView<'_, F> {
    fn redraw(&mut self) {
        *self.needs_redraw = true;
    }
}
