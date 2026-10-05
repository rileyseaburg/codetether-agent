//! Reasoning levels reported by the authenticated Codex model catalog.

#[path = "reasoning_catalog_seed.rs"]
mod seed;

/// Return supported wire-level efforts for a Codex model.
///
/// # Arguments
///
/// * `model` — Model identifier, optionally provider-prefixed or effort-suffixed.
///
/// # Returns
///
/// Returns account-advertised efforts, or offline defaults before discovery.
/// Unknown models return an empty vector.
///
/// # Examples
///
/// ```
/// use codetether_agent::provider::openai_codex::reasoning_catalog::supported_levels;
/// assert!(supported_levels("gpt-5.6-sol").contains(&"ultra"));
/// ```
pub fn supported_levels(model: &str) -> Vec<&'static str> {
    let model = base_model(model);
    let model = model
        .strip_suffix("-ultrafast")
        .or_else(|| model.strip_suffix("-fast"))
        .unwrap_or(model);
    if let Some(entry) = super::model_discovery::model(model) {
        return entry
            .supported_reasoning_levels
            .iter()
            .filter_map(|level| super::thinking_level::ThinkingLevel::parse(&level.effort))
            .map(super::thinking_level::ThinkingLevel::as_str)
            .collect();
    }
    seed::levels(model).to_vec()
}

/// Report whether a model accepts a reasoning-effort wire value.
///
/// # Arguments
///
/// * `model` — Model identifier to inspect.
/// * `effort` — Wire-level reasoning effort such as `"high"`.
///
/// # Returns
///
/// Returns `true` only when the catalog lists the effort for the model.
///
/// # Examples
///
/// ```
/// use codetether_agent::provider::openai_codex::reasoning_catalog::supports;
/// assert!(supports("gpt-5.5", "high"));
/// assert!(!supports("gpt-5.5", "ultra"));
/// ```
pub fn supports(model: &str, effort: &str) -> bool {
    supported_levels(model).contains(&effort)
}

/// Report whether a model belongs to the GPT-5.6 Sol/Terra/Luna family.
///
/// Accepts provider-prefixed, Bedrock `openai.` and Codex `-fast` forms.
pub fn is_gpt_56(model: &str) -> bool {
    let base = base_model(model);
    let base = base.strip_prefix("openai.").unwrap_or(base);
    let base = base.strip_suffix("-fast").unwrap_or(base);
    matches!(base, "gpt-5.6-sol" | "gpt-5.6-terra" | "gpt-5.6-luna")
}

fn base_model(model: &str) -> &str {
    let model = model.rsplit('/').next().unwrap_or(model);
    model.split(':').next().unwrap_or(model)
}

#[cfg(test)]
#[path = "reasoning_catalog_tests.rs"]
mod tests;
