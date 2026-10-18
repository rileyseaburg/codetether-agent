//! Completion request validation, provider selection and dispatch.
use super::{Host, Registry};
use axum::{Json, response::Response};
use codetether_openai_adapter::{OpenAiApiError, openai_bad_request, openai_internal_error};
use codetether_openai_wire::OpenAiChatCompletionRequest;

pub(super) async fn complete<H: Host>(
    Json(req): Json<OpenAiChatCompletionRequest>,
) -> Result<Response, OpenAiApiError> {
    let is_stream = req.stream.unwrap_or(false);
    if req.model.trim().is_empty() {
        return Err(openai_bad_request("`model` is required"));
    }
    if req.messages.is_empty() {
        return Err(openai_bad_request("`messages` must not be empty"));
    }

    let registry = H::load_registry().await.map_err(|error| {
        tracing::error!(error = %error, "Failed to load providers from Vault");
        openai_internal_error(format!("failed to load providers: {error}"))
    })?;

    let providers = registry.list();
    let (selected_provider, model_id) = codetether_openai_adapter::select_request_model(
        &req.model,
        &providers,
        req.service_tier.as_deref(),
        req.reasoning_effort.as_deref(),
    )?;
    let provider = registry.get(&selected_provider).ok_or_else(|| {
        openai_internal_error(format!(
            "Provider `{selected_provider}` was available but could not be loaded"
        ))
    })?;

    let completion_request = codetether_openai_adapter::completion_request(req, &model_id)?;

    crate::serve(
        provider,
        completion_request,
        is_stream,
        selected_provider,
        model_id,
    )
    .await
}
