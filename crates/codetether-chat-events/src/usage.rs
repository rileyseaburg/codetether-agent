//! Usage-report session events.

use crate::UsageView;
use codetether_chat_model::MessageUsage;
use codetether_session_events::SessionEvent;

pub use crate::attach::attach_usage_to_last_completion_message;

/// Apply completion usage metrics and presentation.
/// # Arguments
/// Receives frontend state and the owned event or payload fields.
/// # Returns
/// Updates the supplied frontend state in event order.
/// # Examples
/// ```no_run
/// # fn example(view: &mut impl codetether_chat_events::UsageView) {
/// let _ = codetether_chat_events::usage::report(view, String::new(), 0, 0, 0);
/// # }
/// ```
pub fn report(
    view: &mut impl UsageView,
    model: String,
    prompt_tokens: usize,
    completion_tokens: usize,
    duration_ms: u64,
) {
    view.set_model(model.clone());
    view.set_latency(duration_ms);
    view.set_prompt_tokens(prompt_tokens);
    view.set_output_tokens(completion_tokens);
    attach_usage_to_last_completion_message(
        view.messages(),
        MessageUsage {
            model: model.clone(),
            prompt_tokens,
            completion_tokens,
            duration_ms,
        },
    );
    view.set_status(format!(
        "Completed with model {model} • {prompt_tokens} in / {completion_tokens} out • {duration_ms} ms"
    ));
}

/// Apply completion usage metrics and presentation.
/// # Arguments
/// Receives frontend state and the owned event or payload fields.
/// # Returns
/// Unrelated events are returned unchanged; matching events are consumed.
/// # Examples
/// ```no_run
/// # fn example<S>(view: &mut impl codetether_chat_events::UsageView, event: codetether_session_events::SessionEvent<S>) {
/// let _ = codetether_chat_events::usage::handle_event(view, event);
/// # }
/// ```
pub fn handle_event<S>(view: &mut impl UsageView, evt: SessionEvent<S>) -> Option<SessionEvent<S>> {
    match evt {
        SessionEvent::UsageReport {
            model,
            prompt_tokens,
            completion_tokens,
            duration_ms,
        } => report(view, model, prompt_tokens, completion_tokens, duration_ms),
        other => return Some(other),
    }
    None
}
