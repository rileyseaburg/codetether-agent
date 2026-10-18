//! Serve selected provider completions as OpenAI-compatible HTTP responses.
//!
//! serve owns completion IDs, timestamps, provider calls and response envelopes.
//! The caller retains provider lookup and request validation.
mod complete;
mod response;
mod stream;
mod transport;
use axum::response::Response;
use codetether_openai_adapter::{OpenAiApiError, make_openai_model_id};
use codetether_provider_core::{CompletionRequest, Provider};
use std::sync::Arc;
/// Execute a prepared completion request and produce its HTTP response.
/// # Arguments
/// * `provider` - Selected provider implementation.
/// * `completion_request` - Validated provider request.
/// * `is_stream` - Whether to return SSE output.
/// * `selected_provider` - Provider label used in logs and response IDs.
/// * `model_id` - Provider-local model identifier.
/// # Returns
/// A complete JSON response or an SSE response.
/// # Errors
/// Returns the OpenAI internal-error envelope when the initial provider call fails.
/// # Examples
/// ```no_run
/// # async fn example(provider: std::sync::Arc<dyn codetether_provider_core::Provider>, request: codetether_provider_core::CompletionRequest) {
/// let _ = codetether_openai_serving::serve(provider, request, false, "provider".into(), "model".into()).await;
/// # }
/// ```
pub async fn serve(
    provider: Arc<dyn Provider>,
    completion_request: CompletionRequest,
    is_stream: bool,
    selected_provider: String,
    model_id: String,
) -> Result<Response, OpenAiApiError> {
    let chat_id = format!("chatcmpl-{}", uuid::Uuid::new_v4());
    let now = chrono::Utc::now().timestamp();
    let openai_model = make_openai_model_id(&selected_provider, &model_id);
    if is_stream {
        return stream::serve(
            provider,
            completion_request,
            selected_provider,
            model_id,
            chat_id,
            now,
            openai_model,
        )
        .await;
    }
    complete::serve(
        provider,
        completion_request,
        selected_provider,
        model_id,
        chat_id,
        now,
        openai_model,
    )
    .await
}
