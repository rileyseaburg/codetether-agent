//! Build an ephemeral [`CompletionRequest`] for `/ask`.
//!
//! Clones the session's messages, appends a guarded side-question user
//! message (no tools, single reply), and resolves the current session's
//! model to a provider via [`crate::autochat::model_rotation`].

use std::sync::Arc;

use crate::provider::{
    CompletionRequest, ContentPart, Message, Provider, ProviderRegistry, Role,
};
use crate::session::Session;

/// Resolve the session's provider and build the ephemeral request.
///
/// Returns [`None`] when no provider can be resolved for the session's
/// configured model, leaving the caller to report a status error.
pub(super) fn build_request(
    session: &Session,
    registry: &Arc<ProviderRegistry>,
    question: &str,
) -> Option<(Arc<dyn Provider>, CompletionRequest)> {
    let model_ref = session
        .metadata
        .model
        .clone()
        .unwrap_or_else(|| "bedrock".to_string());
    let (provider, model) =
        crate::autochat::model_rotation::resolve_provider_for_model_autochat(registry, &model_ref)?;
    let mut messages = session.messages.clone();
    messages.push(Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: format!(
                "[SIDE QUESTION — answer from context only, no tools, single reply]\n{question}"
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
