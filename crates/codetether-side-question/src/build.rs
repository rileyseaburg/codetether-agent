//! Build an ephemeral [`CompletionRequest`] for `/ask`.
//!
//! Clones the session's messages, appends a guarded side-question user
//! message (no tools, single reply), and resolves the current session's
//! model to a provider via the application registry.

use std::sync::Arc;

use crate::Registry;
use codetether_provider_core::{CompletionRequest, ContentPart, Message, Provider, Role};

/// Resolve the session's provider and build the ephemeral request.
///
/// Returns [`None`] when no provider can be resolved for the session's
/// configured model, leaving the caller to report a status error.
pub(super) fn build_request<R: Registry>(
    history: &[Message],
    current_model: Option<&str>,
    registry: &Arc<R>,
    question: &str,
) -> Option<(Arc<dyn Provider>, CompletionRequest)> {
    let model_ref = current_model
        .map(str::to_owned)
        .unwrap_or_else(|| "bedrock".to_string());
    let (provider, model) = R::resolve(registry, &model_ref)?;
    let mut messages = history.to_vec();
    messages.push(Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: format!(
                "[SIDE QUESTION — answer from context only, no tools, single reply. \
                 This reply cannot resume goal execution. Never claim work is resuming; \
                 direct continuation requests to /continue.]\n{question}"
            ),
        }],
    });
    Some((
        provider,
        CompletionRequest {
            messages,
            tools: vec![],
            model,
            temperature: None,
            top_p: None,
            max_tokens: Some(1024),
            stop: vec![],
        },
    ))
}
