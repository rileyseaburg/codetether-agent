//! Builds the OpenRouter `reasoning` request object.

use super::reasoning_levels;
use serde_json::{Value, json};

/// Returns the `reasoning` object to send for `model`, or `None` to omit it.
///
/// `None` is returned when no override is configured, when the value is not a
/// level OpenRouter accepts, or when `none` was requested for a model that
/// mandates reasoning — sending it anyway would fail the turn with HTTP 400.
///
/// # Arguments
///
/// * `model` — Target model ID, e.g. `x-ai/grok-4.6`.
/// * `configured` — Raw override value, typically from `runtime_config`.
pub(super) fn reasoning_object(model: &str, configured: Option<&str>) -> Option<Value> {
    let effort = reasoning_levels::normalize(configured?)?;
    if effort == "none" && reasoning_levels::requires_reasoning(model) {
        tracing::debug!(
            provider = "openrouter",
            model = %model,
            "Model mandates reasoning; omitting reasoning.effort=none"
        );
        return None;
    }
    Some(json!({ "effort": effort }))
}

/// Applies the reasoning object to a request body when one is warranted.
///
/// `include_reasoning` is requested alongside it so reasoning text streams
/// back and surfaces as thinking output rather than being silently dropped.
pub(super) fn apply(body: &mut Value, model: &str, configured: Option<&str>) {
    let Some(reasoning) = reasoning_object(model, configured) else {
        return;
    };
    tracing::debug!(
        provider = "openrouter",
        model = %model,
        reasoning = %reasoning,
        "Applying OpenRouter reasoning effort"
    );
    body["reasoning"] = reasoning;
    body["include_reasoning"] = json!(true);
}

#[cfg(test)]
#[path = "reasoning_request_tests.rs"]
mod tests;
