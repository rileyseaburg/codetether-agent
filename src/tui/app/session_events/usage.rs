//! Usage-report session events.

mod attach;

use crate::session::SessionEvent;
use crate::tui::app::state::App;
use crate::tui::chat::message::MessageUsage;

pub use attach::attach_usage_to_last_completion_message;

pub(super) fn report(
    app: &mut App,
    model: String,
    prompt_tokens: usize,
    completion_tokens: usize,
    duration_ms: u64,
) {
    app.state.last_completion_model = Some(model.clone());
    app.state.last_completion_latency_ms = Some(duration_ms);
    app.state.last_completion_prompt_tokens = Some(prompt_tokens);
    app.state.last_completion_output_tokens = Some(completion_tokens);
    attach_usage_to_last_completion_message(
        &mut app.state.messages,
        MessageUsage {
            model: model.clone(),
            prompt_tokens,
            completion_tokens,
            duration_ms,
        },
    );
    app.state.status = format!(
        "Completed with model {model} • {prompt_tokens} in / {completion_tokens} out • {duration_ms} ms"
    );
}

pub(super) fn handle_event(app: &mut App, evt: SessionEvent) -> Option<SessionEvent> {
    match evt {
        SessionEvent::UsageReport {
            model,
            prompt_tokens,
            completion_tokens,
            duration_ms,
        } => report(app, model, prompt_tokens, completion_tokens, duration_ms),
        other => return Some(other),
    }
    None
}
