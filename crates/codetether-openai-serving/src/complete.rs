//! Request a complete provider response with existing error reporting.
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
    let completion = provider
        .complete(completion_request)
        .await
        .map_err(|error| {
            tracing::warn!(
                provider = %selected_provider,
                model = %model_id,
                error = %error,
                "Completion request failed"
            );
            openai_internal_error(error.to_string())
        })?;

    Ok(crate::response::build(
        completion,
        chat_id,
        now,
        openai_model,
    ))
}
