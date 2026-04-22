//! `/ask` — ephemeral side question handler.
//!
//! Runs a single, tool-less completion against the session's current
//! provider using the full conversation as context, renders the answer
//! as an in-chat system message, and **never mutates session history**.
//! Mirrors Claude Code's `/btw`: full context, no tools, single reply,
//! ephemeral. Works across every provider in the registry.

mod build;
mod execute;
mod extract;

use std::sync::Arc;

use crate::provider::ProviderRegistry;
use crate::session::Session;
use crate::tui::app::state::App;

/// Run the `/ask` side question and render the answer inline.
///
/// # Arguments
///
/// * `app` — TUI state; receives the answer + status updates.
/// * `session` — current conversation; **read-only**, never mutated.
/// * `registry` — provider registry. [`None`] ⇒ status-only error.
/// * `question` — trimmed text after `/ask`. Empty ⇒ usage hint.
///
/// # Errors
///
/// Provider errors are surfaced via `app.state.status`; this function
/// does not return a `Result`.
pub(super) async fn run_ask(
    app: &mut App,
    session: &Session,
    registry: Option<&Arc<ProviderRegistry>>,
    question: &str,
) {
    if question.is_empty() {
        app.state.status =
            "Usage: /ask <question> — ephemeral, full context, no tools, not saved.".to_string();
        return;
    }
    let Some(registry) = registry else {
        app.state.status = "/ask: no provider configured".to_string();
        return;
    };
    let Some((provider, request)) = build::build_request(session, registry, question) else {
        app.state.status = "/ask: cannot resolve provider".to_string();
        return;
    };
    execute::run(app, provider, request).await;
}
