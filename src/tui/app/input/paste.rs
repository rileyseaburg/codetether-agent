//! Paste event handling for the TUI.
//!
//! Normalises line endings and dispatches pasted text to the
//! correct input buffer based on the active view mode.
//!
//! # Examples
//!
//! ```ignore
//! handle_paste(&mut app, "multi\nline").await;
//! ```

use crate::tui::app::state::App;

#[path = "paste_host.rs"]
mod host;

/// Handle pasted text from the terminal.
///
/// Normalises line endings, then dispatches to symbol search,
/// bus filter, model filter, sessions filter, or chat input.
///
/// # Examples
///
/// ```ignore
/// handle_paste(&mut app, "pasted\ntext").await;
/// ```
pub async fn handle_paste(app: &mut App, text: &str) {
    codetether_paste_buffer::routing::handle(app, text);
}

/// Insert pasted text into the chat input buffer.
///
/// Shared by the bracketed-paste event path and the Ctrl+V clipboard
/// path so both get identical image-data-URL extraction and sidecar
/// summarisation behavior. Expects line endings already normalised.
pub fn paste_into_chat(app: &mut App, normalized: &str) {
    codetether_paste_buffer::input::paste_into_chat(app, normalized);
}

#[path = "paste_routing_host.rs"]
mod routing_host;
