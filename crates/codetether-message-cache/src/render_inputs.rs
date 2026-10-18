//! Current application values affecting rendered-line reuse.
/// Cache validation inputs, independent of the application container.
///
/// # Examples
/// ```
/// let inputs = codetether_message_cache::RenderInputs::default();
/// assert!(!inputs.pending_tool);
/// ```
#[derive(Default, Clone, Copy)]
pub struct RenderInputs {
    /// Whether a live tool timer requires a fresh render.
    pub pending_tool: bool,
    /// Current number of displayed messages.
    pub messages_len: usize,
    /// Current tool-panel scroll position.
    pub tool_preview_scroll: usize,
    /// Whether the assistant is processing a turn.
    pub processing: bool,
}
