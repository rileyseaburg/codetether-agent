//! Static known Z.AI models merged with dynamic discovery.
//!
//! [`merge_known`] preserves discovered entries and appends missing catalog
//! entries. It excludes the deprecated pony-alpha-2 model case-insensitively.

#[path = "entries.rs"]
mod entries;

use super::ModelInfo;
use entries::{Entry, KNOWN_MODELS};

#[cfg(test)]
#[path = "tests.rs"]
mod tests;

/// Combine discovered models with the known Z.AI catalog.
///
/// # Arguments
/// * `models` - Discovered models, whose ordering and metadata are retained.
///
/// # Returns
/// Discovered and known models with deprecated entries removed.
///
/// # Examples
/// ```
/// let models = codetether_provider_zai::catalog::merge_known(vec![]);
/// assert!(models.iter().any(|model| model.id == "glm-5.3"));
/// ```
pub fn merge_known(mut models: Vec<ModelInfo>) -> Vec<ModelInfo> {
    models.retain(|model| !model.id.eq_ignore_ascii_case("pony-alpha-2"));
    for entry in KNOWN_MODELS {
        if !models.iter().any(|model| model.id == entry.0) {
            models.push(info(entry));
        }
    }
    models
}

fn info(entry: &Entry) -> ModelInfo {
    ModelInfo {
        id: entry.0.into(),
        name: entry.1.into(),
        provider: "zai".into(),
        context_window: entry.2,
        max_output_tokens: Some(entry.3),
        supports_vision: false,
        supports_tools: true,
        supports_streaming: true,
        input_cost_per_million: entry.4,
        output_cost_per_million: entry.5,
    }
}
