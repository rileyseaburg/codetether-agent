use std::time::Instant;
/// Borrowed input and timing values for paste-burst detection.
/// # Examples
/// ```
/// let view = codetether_paste_buffer::burst::View {
///     in_chat: true, input: "hello", cursor: 5, last_key_at: None,
/// };
/// assert!(!codetether_paste_buffer::burst::enter_is_likely(&view, false));
/// ```
pub struct View<'a> {
    /// Whether the current view accepts chat input.
    pub in_chat: bool,
    /// Current input text.
    pub input: &'a str,
    /// Cursor position in characters.
    pub cursor: usize,
    /// Time of the last dispatched ordinary key.
    pub last_key_at: Option<Instant>,
}
