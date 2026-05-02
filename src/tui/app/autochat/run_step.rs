//! Single-step provider call + model-ref resolution for [`run_relay`].
//!
//! Lives in its own module so [`run`] stays under the global per-file
//! line budget after the persona-state wiring landed alongside it.
//!
//! [`run_relay`]: super::run::run_relay
//! [`run`]: super::run

use super::persona::Persona;
use super::step_request::build_step_request;
use super::summary::summarize_response;

/// Run one persona turn against the resolved provider.
pub async fn run_step(
    provider: &dyn crate::provider::Provider,
    resolved_model: &str,
    persona: &Persona,
    task: &str,
    baton: &str,
) -> anyhow::Result<String> {
    let request = build_step_request(resolved_model.to_string(), persona, task, baton);
    let response = provider.complete(request).await?;
    Ok(summarize_response(response))
}

/// Default to the autochat model alias when the caller passed an empty
/// string, otherwise honour their explicit selection.
pub fn resolve_model_ref(model: String) -> String {
    let trimmed = model.trim();
    if trimmed.is_empty() {
        "codetether-autochat".to_string()
    } else {
        trimmed.to_string()
    }
}
