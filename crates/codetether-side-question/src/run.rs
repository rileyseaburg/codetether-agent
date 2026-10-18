//! Validate and dispatch an ephemeral side question.
use crate::{Registry, View, build, execute};
use codetether_provider_core::Message;
use std::sync::Arc;
/// Ask one tool-free question using existing conversation context.
/// # Arguments
/// * `view` - Ephemeral presentation bindings.
/// * `history` - Read-only conversation context, cloned into the request.
/// * `model` - Current optional session model.
/// * `registry` - Existing provider registry, if available.
/// * `question` - Trimmed question text.
/// # Returns
/// True only when an answer was displayed; false after reporting usage or failure.
/// # Examples
/// ```no_run
/// # async fn example<V: codetether_side_question::View, R: codetether_side_question::Registry>(view: &mut V, registry: &std::sync::Arc<R>) {
/// let answered = codetether_side_question::run_ask(view, &[], None, Some(registry), "Why?").await;
/// # }
/// ```
pub async fn run_ask<V: View, R: Registry>(
    view: &mut V,
    history: &[Message],
    model: Option<&str>,
    registry: Option<&Arc<R>>,
    question: &str,
) -> bool {
    if question.is_empty() {
        view.set_status(
            "Usage: /ask <question> — ephemeral, full context, no tools, not saved.".to_string(),
        );
        return false;
    }
    let Some(registry) = registry else {
        view.set_status("/ask: no provider configured".to_string());
        return false;
    };
    let Some((provider, request)) = build::build_request(history, model, registry, question) else {
        view.set_status("/ask: cannot resolve provider".to_string());
        return false;
    };
    execute::run(view, provider, request).await
}
