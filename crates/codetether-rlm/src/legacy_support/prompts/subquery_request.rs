//! Shared request policy for legacy DSL and structured sub-queries.
use codetether_provider_core::CompletionRequest;
/// Build the shared legacy sub-query request and bounded context messages.
///
/// # Arguments
/// * `model` - Provider-local model identifier.
/// * `context` - Selected context to inspect.
/// * `query` - Focused analysis question.
/// # Returns
/// A request with the original temperature and 500-token output limit.
/// # Examples
/// ```
/// let request = codetether_rlm::legacy_support::prompts::subquery_request("model", "text", "why?");
/// assert_eq!(request.max_tokens, Some(500));
/// assert_eq!(request.messages.len(), 2);
/// ```
pub fn subquery_request(model: &str, context: &str, query: &str) -> CompletionRequest {
    let messages = super::subquery_messages::subquery_messages(context, query);
    CompletionRequest {
        messages,
        tools: vec![],
        model: model.to_string(),
        temperature: Some(0.3),
        top_p: None,
        max_tokens: Some(500),
        stop: vec![],
    }
}
