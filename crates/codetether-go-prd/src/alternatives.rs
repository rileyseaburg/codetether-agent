//! Choose available alternative models in the established preference order.

use crate::Registry;

/// Get ordered list of alternative models for PRD generation.
/// Excludes the primary model to avoid redundant attempts.
pub(crate) fn get_model_alternatives<'a>(
    primary_model: &'a str,
    registry: &'a dyn Registry,
) -> Vec<(&'a str, String)> {
    let mut alternatives = Vec::new();

    // Known good models for PRD generation (reasoning-capable models work best)
    // Ordered by preference - most capable first
    let model_preferences = [
        ("openai", "gpt-4o"),
        ("openai", "gpt-4o-mini"),
        ("anthropic", "claude-sonnet-4-20250514"),
        ("anthropic", "claude-3-5-sonnet-20241022"),
        ("google", "gemini-2.0-flash"),
        ("openrouter", "openai/gpt-4o"),
        ("openrouter", "anthropic/claude-3.5-sonnet"),
        ("zai", "glm-5"),
    ];

    for (provider, model) in model_preferences {
        // Skip if this is the primary model
        let model_string = format!("{}/{}", provider, model);
        if model_string == primary_model || model == primary_model {
            continue;
        }

        // Only add if provider is available in registry
        if registry.get(provider).is_some() {
            alternatives.push((provider, model_string));
        }
    }

    alternatives
}
