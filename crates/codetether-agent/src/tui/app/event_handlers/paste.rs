//! Paste event forwarding for the TUI.
//!
//! Wraps the terminal paste bracket event and delegates to
//! [`crate::tui::app::input::handle_paste`].
//!
//! # Examples
//!
//! ```ignore
//! handle_paste_event(&mut app, "some text").await;
//! ```

use crate::tui::app::input::handle_paste;
use crate::tui::app::state::App;

#[path = "question_key.rs"]
pub(super) mod question_key;

/// Forward a paste event from the terminal to input handling.
///
/// Delegates directly to [`crate::tui::app::input::handle_paste`].
///
/// # Examples
///
/// ```ignore
/// handle_paste_event(&mut app, "pasted text").await;
/// ```
pub async fn handle_paste_event(app: &mut App, text: &str) {
    if question_key::paste(app, text) {
        return;
    }
    handle_paste(app, text).await;
}
