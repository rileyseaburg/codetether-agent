//! Borrowed composer content and display state.
/// Values required to paint the input textbox and cursor.
///
/// # Examples
/// ```
/// let view = codetether_composer_view::ComposerView {
///     text: "hello", cursor: 5, processing: false, command_mode: false };
/// assert_eq!(view.cursor, 5);
/// ```
pub struct ComposerView<'a> {
    /// Current input text.
    pub text: &'a str,
    /// Cursor position in Unicode scalar values.
    pub cursor: usize,
    /// Whether a turn is running.
    pub processing: bool,
    /// Whether the input is being treated as a command.
    pub command_mode: bool,
}
