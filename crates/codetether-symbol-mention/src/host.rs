//! Composer operations required by the symbol mention flow.
use codetether_symbol_search::SymbolSearchState;
use std::ops::Range;
/// Supplies text editing, picker state and status for symbol mentions.
///
/// # Implementors
/// The application implements this interface using its existing input editor.
/// Cursor positions and replacement ranges are Unicode character indices.
pub trait Composer {
    /// Current composer text.
    fn input(&self) -> &str;
    /// Current cursor position in Unicode characters.
    fn cursor(&self) -> usize;
    /// Current symbol picker state.
    fn picker(&self) -> &SymbolSearchState;
    /// Mutable symbol picker state.
    fn picker_mut(&mut self) -> &mut SymbolSearchState;
    /// Insert one character using the host's normal editing path.
    fn insert_char(&mut self, character: char);
    /// Insert text using the host's normal editing path.
    fn insert_text(&mut self, text: &str);
    /// Replace a character range using the host's normal editing path.
    fn replace_input_chars(&mut self, range: Range<usize>, replacement: &str);
    /// Replace the composer's status message.
    fn set_status(&mut self, status: String);
}
