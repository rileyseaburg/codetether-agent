//! Invoke the selected provider and parse its relay objective response.
use crate::{PlannedOkrDraft, parse::extract_json_payload};
use codetether_provider_core::Provider;
use std::sync::Arc;

/// Request a relay objective from a caller-selected provider and model.
/// # Arguments
/// * `task` - Relay work to plan.
/// * `model_name` - Provider-local model name.
/// * `agent_count` - Team size included in the prompt.
/// * `provider` - Already-resolved provider instance.
/// # Returns
/// A deserialized draft, or None after provider failure or invalid JSON output.
/// # Examples
/// ```no_run
/// # async fn example(provider: &std::sync::Arc<dyn codetether_provider_core::Provider>) {
/// let _ = codetether_relay_okr::draft("build feature", "model".into(), 3, provider).await;
/// # }
/// ```
pub async fn draft(
    task: &str,
    model_name: String,
    agent_count: usize,
    provider: &Arc<dyn Provider>,
) -> Option<PlannedOkrDraft> {
    let model_name_for_log = model_name.clone();

    let request = crate::request::build(task, model_name, agent_count);

    let response = provider.complete(request).await.ok()?;
    let text = response
        .message
        .content
        .iter()
        .filter_map(|part| match part {
            codetether_provider_core::ContentPart::Text { text }
            | codetether_provider_core::ContentPart::Thinking { text, .. } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n");

    tracing::debug!(
        model = %model_name_for_log,
        response_len = text.len(),
        response_preview = %text.chars().take(500).collect::<String>(),
        "OKR draft model response"
    );

    extract_json_payload::<PlannedOkrDraft>(&text)
}
