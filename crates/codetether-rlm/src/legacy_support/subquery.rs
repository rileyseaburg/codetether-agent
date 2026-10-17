//! Complete a legacy sub-query using the shared request and text-response policy.
use anyhow::Result;
use codetether_provider_core::{ContentPart, Provider};

/// Run one sub-query and collect its text and total token usage.
///
/// # Arguments
/// * `provider` - Already selected provider handle.
/// * `model` - Provider-local model identifier.
/// * `context` - Selected text to analyze.
/// * `query` - Focused question about that context.
/// # Returns
/// Concatenated text parts and the provider-reported total token count.
/// # Errors
/// Propagates the provider's completion error without recording a result.
/// # Examples
/// ```no_run
/// # async fn example(provider: &dyn codetether_provider_core::Provider) -> anyhow::Result<()> {
/// let (answer, tokens) = codetether_rlm::legacy_support::subquery::complete(
///     provider, "model", "context", "question",
/// ).await?;
/// let _ = (answer, tokens);
/// # Ok(()) }
/// ```
pub async fn complete(
    provider: &dyn Provider,
    model: &str,
    context: &str,
    query: &str,
) -> Result<(String, usize)> {
    let response = provider
        .complete(super::prompts::subquery_request(model, context, query))
        .await?;
    let answer = response
        .message
        .content
        .iter()
        .filter_map(|p| match p {
            ContentPart::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("");
    Ok((answer, response.usage.total_tokens))
}
