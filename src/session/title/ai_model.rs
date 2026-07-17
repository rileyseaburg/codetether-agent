//! Model-backed title synthesis: the cheap LLM call and output cleaning.

use anyhow::Result;

use crate::provider::Provider;

use super::super::helper::text::truncate_with_ellipsis;
use super::ai_request::title_request;

/// Ask the selected provider and model for a short title.
pub(super) async fn title_from_model(
    provider: &dyn Provider,
    model: &str,
    seed: &str,
) -> Result<Option<String>> {
    let response = provider
        .complete(title_request(model.to_string(), seed))
        .await?;
    let raw = crate::session::helper::text::extract_text_content(&response.message.content);
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
