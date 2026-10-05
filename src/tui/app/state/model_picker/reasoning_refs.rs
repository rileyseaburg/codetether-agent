//! OpenAI Codex reasoning variants shown by the TUI model picker.

use crate::provider::openai_codex::{reasoning_catalog, service_tier_catalog};

pub(super) fn expand(provider: &str, model_ref: String) -> Vec<String> {
    if provider != "openai-codex" {
        return vec![model_ref];
    }
    let levels = reasoning_catalog::supported_levels(&model_ref);
    let mut variants = Vec::with_capacity((levels.len() + 1) * 2);
    append_family(&mut variants, &model_ref, &levels);
    for suffix in service_tier_catalog::suffixes(&model_ref) {
        append_family(&mut variants, &format!("{model_ref}{suffix}"), &levels);
    }
    variants
}

fn append_family(variants: &mut Vec<String>, model: &str, levels: &[&str]) {
    variants.push(model.to_string());
    variants.extend(levels.iter().map(|level| format!("{model}:{level}")));
}

#[cfg(test)]
#[path = "reasoning_refs_tests.rs"]
mod tests;
