//! Repository prompts and autonomous tool definitions.
use crate::Host;
use codetether_provider_core::{Provider, ToolDefinition};
use std::{path::Path, sync::Arc};
pub(super) fn build<H: Host>(
    provider: &Arc<dyn Provider>,
    model: &str,
    directory: &Path,
) -> (String, Arc<H::Registry>, Vec<ToolDefinition>) {
    let system = H::system_prompt(directory);
    let registry = H::registry(Arc::clone(provider), model.to_string());
    let definitions = H::definitions(&registry)
        .into_iter()
        .filter(|tool| tool.name != "question")
        .collect();
    (system, registry, definitions)
}
