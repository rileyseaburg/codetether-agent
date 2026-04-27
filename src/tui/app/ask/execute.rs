//! Execute a resolved `/ask` request and render the answer.
//!
//! Isolated so [`super::run_ask`] stays small enough to satisfy the
//! 50-line module budget while still carrying full rustdoc.

use std::sync::Arc;

use super::extract;
use crate::provider::{CompletionRequest, Provider};
use crate::tui::app::provider_error::user_facing_error;
use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};

/// Send `request` to `provider`, then render the reply or the error.
///
/// On success appends a [`MessageType::System`] chat message of the
/// form `"/ask → <reply>"`; on failure only `app.state.status` is
/// updated. Never mutates session history.
pub(super) async fn run(app: &mut App, provider: Arc<dyn Provider>, request: CompletionRequest) {
    app.state.status = "/ask: asking…".to_string();
    match provider.complete(request).await {
        Ok(resp) => {
            let text = extract::extract_text(&resp.message);
            app.state.messages.push(ChatMessage::new(
                MessageType::System,
                format!("/ask → {text}"),
            ));
            app.state.status = "/ask: answered".to_string();
            app.state.scroll_to_bottom();
        }
        Err(err) => {
            let shown = user_facing_error(&err.to_string());
            app.state.status = format!("/ask failed: {shown}");
        }
    }
}
