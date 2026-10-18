//! Interactive ask command preflight with unchanged usage presentation.
use crate::{Registry, View};
use codetether_provider_core::Message;
use std::sync::Arc;
/// Trim and run a side question, or display interactive usage guidance.
/// # Arguments
/// * `view` — Ephemeral output presentation.
/// * `history` — Current read-only conversation context.
/// * `model` — Optional current model selection.
/// * `registry` — Available providers.
/// * `rest` — Raw text following the ask command.
/// # Returns
/// Displays usage or delegates the question without mutating session history.
/// # Examples
/// ```no_run
/// # async fn ask<R: codetether_side_question::Registry>(view: &mut impl codetether_side_question::View) {
/// codetether_side_question::command::execute::<_, R>(view, &[], None, None, "").await;
/// # }
/// ```
pub async fn execute<V: View, R: Registry>(
    view: &mut V,
    history: &[Message],
    model: Option<&str>,
    registry: Option<&Arc<R>>,
    rest: &str,
) {
    let question = rest.trim();
    if question.is_empty() {
        view.set_status(
            "Usage: /ask <question> — ephemeral side question (full context, no tools, not saved)"
                .to_string(),
        );
        view.push_system_message("`/ask <question>` runs an ephemeral side query with full context but no tools, and is not saved to the session.".to_string());
        view.scroll_to_bottom();
        return;
    }
    crate::run_ask(view, history, model, registry, question).await;
}
