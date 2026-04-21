//! `/ask` — ephemeral side question handler.
//!
//! Runs a single, tool-less completion against the session's current
//! provider using the full conversation as context, renders the answer
//! as an in-chat system message, and **never mutates session history**.
//! Mirrors Claude Code's `/btw`: full context, no tools, single reply,
//! ephemeral.

mod build;
mod extract;

use std::sync::Arc;

use crate::provider::ProviderRegistry;
use crate::session::Session;
use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};

/// Run the `/ask` side question and render the answer inline.
///
/// `question` is the text after `/ask`. Awaits the provider, then
/// pushes a [`MessageType::System`] chat message with the answer.
/// Neither the question nor the answer is appended to
/// [`Session::messages`], so history stays clean.
pub(super) async fn run_ask(
    app: &mut App,
    session: &Session,
    registry: Option<&Arc<ProviderRegistry>>,
    question: &str,
) {
    let Some(registry) = registry else {
        app.state.status = "/ask: no provider configured".to_string();
        return;
    };
    let Some((provider, request)) = build::build_request(session, registry, question) else {
        app.state.status = "/ask: cannot resolve provider".to_string();
        return;
    };
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
            app.state.status = format!("/ask failed: {err}");
        }
    }
}
