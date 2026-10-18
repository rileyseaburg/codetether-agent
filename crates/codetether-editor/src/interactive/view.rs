//! Embedded editor and fuzzy finder state access.
use crate::FileBuffer;
use codetether_fuzzy_finder::FuzzyFindState;
/// Client state operations used when opening an editor document.
/// # Implementors
/// Native UIs provide their existing editor, finder and view-mode fields.
pub trait View {
    /// Access the optional fuzzy finder state.
    fn finder(&mut self) -> &mut Option<FuzzyFindState>;
    /// Store the opened editor buffer.
    fn set_buffer(&mut self, buffer: FileBuffer);
    /// Reset vertical and horizontal editor scrolling.
    fn reset_editor_scroll(&mut self);
    /// Select the embedded editor view.
    fn show_editor(&mut self);
    /// Replace the interactive status text.
    fn set_status(&mut self, status: String);
}
