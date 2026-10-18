//! Native binding for shared text-paste sidecars.
use crate::tui::app::state::AppState;
pub use codetether_paste_buffer::{
    PendingTextPaste, expand_paste_placeholders, format_size, should_summarize,
};
/// Attach pasted content to the application sidecar and return its placeholder.
pub fn attach_paste(state: &mut AppState, content: String) -> String {
    codetether_paste_buffer::attach_paste(&mut state.pending_text_pastes, content)
}
