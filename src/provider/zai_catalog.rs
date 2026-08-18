//! Static known Z.AI models merged with dynamic discovery.

#[path = "zai_catalog_entries.rs"]
mod entries;

use super::ModelInfo;
use entries::{Entry, KNOWN_MODELS};

#[cfg(test)]
#[path = "zai_catalog_tests.rs"]
mod tests;

pub fn merge_known(mut models: Vec<ModelInfo>) -> Vec<ModelInfo> {
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
