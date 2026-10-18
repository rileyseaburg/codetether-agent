//! Model-backed title synthesis: the cheap LLM call and output cleaning.

use anyhow::Result;

use codetether_provider_core::Provider;

use super::ai_request::title_request;
use codetether_context_policy::transcript::truncate_with_ellipsis;

/// Ask the selected provider and model for a short title.
/// # Arguments
/// `provider` and `model` select the request target; `seed` is user text.
/// # Returns
/// Cleaned first-line title text, or `None` when the response has no usable text.
/// # Errors
/// Propagates provider completion failures.
/// # Examples
/// ```rust,no_run
/// # async fn example(provider: &dyn codetether_provider_core::Provider) -> anyhow::Result<()> {
/// let title = codetether_session_title::title_from_model(provider, "model", "Review storage").await?;
/// # Ok(()) }
/// ```
pub async fn title_from_model(
    provider: &dyn Provider,
    model: &str,
    seed: &str,
) -> Result<Option<String>> {
    let response = provider
        .complete(title_request(model.to_string(), seed))
        .await?;
    let raw =
        codetether_context_policy::transcript::extract_text_content(&response.message.content);
    let cleaned = clean_title(&raw);
    Ok((!cleaned.is_empty()).then_some(cleaned))
}

fn clean_title(raw: &str) -> String {
    let first_line = raw.lines().next().unwrap_or("").trim();
    let trimmed = first_line.trim_matches(['"', '\'', '`', '.', ' ']);
    truncate_with_ellipsis(trimmed, 47)
}

#[cfg(test)]
#[path = "ai_model_tests.rs"]
mod tests;
