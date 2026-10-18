//! Acquire a provider stream and preserve initial request errors.
use axum::response::Response;
use codetether_openai_adapter::{OpenAiApiError, openai_internal_error};
use codetether_provider_core::{CompletionRequest, Provider};
use std::sync::Arc;
pub(crate) async fn serve(
    provider: Arc<dyn Provider>,
    completion_request: CompletionRequest,
    selected_provider: String,
    model_id: String,
    chat_id: String,
    now: i64,
    openai_model: String,
) -> Result<Response, OpenAiApiError> {
    let provider_stream = provider
        .complete_stream(completion_request)
        .await
        .map_err(|error| {
            tracing::warn!(
                provider = %selected_provider,
                model = %model_id,
                error = %error,
                "Streaming completion request failed"
            );
            openai_internal_error(error.to_string())
        })?;

    Ok(crate::transport::response(
        provider_stream,
        chat_id,
        now,
        openai_model,
    ))
}
