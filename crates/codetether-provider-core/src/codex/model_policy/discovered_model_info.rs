//! Selector metadata derived from an account discovery entry.
use super::*;

/// Convert an account-discovered model into selector metadata.
///
/// # Arguments
/// `model` is a parsed account catalog entry.
/// # Returns
/// Metadata with existing context/output fallbacks and the discovered label.
pub fn discovered_model_info(model: &model_discovery::DiscoveredModel) -> ModelInfo {
    let window = model
        .context_window
        .and_then(|w| usize::try_from(w).ok())
        .unwrap_or(272_000);
    let name = model
        .display_name
        .clone()
        .unwrap_or_else(|| model.slug.clone());
    model_info(&model.slug, &name, window, 128_000, false)
}
