//! Borrowed symbol picker and callbacks into an existing text editor.
use crate::Composer;
use codetether_symbol_search::SymbolSearchState;
use std::ops::Range;

/// Connects mention operations to an editor without owning frontend state.
///
/// Editing callbacks preserve the editor's normal cursor, completion and
/// history updates. Ranges and cursor positions use Unicode character indices.
/// # Examples
/// ```
/// # fn bind<C>(view: codetether_symbol_mention::ComposerView<'_, C>) {
/// fn accepts_composer(_: &impl codetether_symbol_mention::Composer) {}
/// accepts_composer(&view);
/// # }
/// ```
pub struct ComposerView<'a, C> {
    /// Existing editor state.
    pub editor: &'a mut C,
    /// Picker storing the query, selection and mention start.
    pub picker: &'a mut SymbolSearchState,
    /// Visible frontend status text.
    pub status: &'a mut String,
    /// Borrow the editor's current input text.
    pub input: fn(&C) -> &str,
    /// Read the editor's cursor in Unicode characters.
    pub cursor: fn(&C) -> usize,
    /// Insert a character through normal editing behavior.
    pub insert_char: fn(&mut C, char),
    /// Insert text through normal editing behavior.
    pub insert_text: fn(&mut C, &str),
    /// Replace a character range through normal editing behavior.
    pub replace: fn(&mut C, Range<usize>, &str),
}

impl<C> Composer for ComposerView<'_, C> {
    fn input(&self) -> &str {
        (self.input)(self.editor)
    }
    fn cursor(&self) -> usize {
        (self.cursor)(self.editor)
    }
    fn picker(&self) -> &SymbolSearchState {
        self.picker
    }
    fn picker_mut(&mut self) -> &mut SymbolSearchState {
        self.picker
    }
    fn insert_char(&mut self, character: char) {
        (self.insert_char)(self.editor, character);
    }
    fn insert_text(&mut self, text: &str) {
        (self.insert_text)(self.editor, text);
    }
    fn replace_input_chars(&mut self, range: Range<usize>, replacement: &str) {
        (self.replace)(self.editor, range, replacement);
    }
    fn set_status(&mut self, status: String) {
        *self.status = status;
    }
}
