//! Model-backed title synthesis: the cheap LLM call and output cleaning.

use anyhow::Result;

use crate::provider::ProviderRegistry;

use super::super::helper::text::truncate_with_ellipsis;
use super::ai_request::title_request;

/// Ask the first available provider's default model for a short title.
///
/// Returns `Ok(None)` when no provider is configured so callers can fall
/// back to the heuristic title.
pub(super) async fn title_from_model(
    registry: &ProviderRegistry,
    seed: &str,
) -> Result<Option<String>> {
    let providers = registry.list();
    let Some(provider_name) = providers.first() else {
        return Ok(None);
    };
    let provider = registry
        .get(provider_name)
        .ok_or_else(|| anyhow::anyhow!("provider {provider_name} missing"))?;
    let model = crate::session::helper::defaults::default_model_for_provider(provider_name);
    let response = provider.complete(title_request(model, seed)).await?;
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
mod tests {
    use super::clean_title;

    #[test]
    fn strips_quotes_and_trailing_period() {
        assert_eq!(clean_title("\"Fix build errors\"."), "Fix build errors");
    }

    #[test]
    fn keeps_only_first_line() {
        assert_eq!(
            clean_title("Add session naming\nextra"),
            "Add session naming"
        );
    }
}
