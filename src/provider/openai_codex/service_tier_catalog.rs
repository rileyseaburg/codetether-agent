//! Codex service tiers: live account capabilities with offline seed defaults.
const FAST_MODELS: &[&str] = &[
    "gpt-6.1-sol",
    "gpt-6-sol",
    "gpt-6-luna",
    "gpt-6-astra",
    "gpt-reserve",
    "gpt-5.4",
    "gpt-5.5",
    "gpt-5.6-sol",
    "gpt-5.6-terra",
    "gpt-5.6-luna",
    "codex-auto-review",
];
/// Whether the model supports the priority-backed Fast alias.
pub(crate) fn supports_fast(model: &str) -> bool {
    supports(model, "priority", FAST_MODELS.contains(&base_model(model)))
}
/// Whether the model supports the Ultrafast service tier.
pub(crate) fn supports_ultrafast(model: &str) -> bool {
    supports(model, "ultrafast", base_model(model) == "gpt-6-astra")
}
fn supports(model: &str, tier: &str, seed: bool) -> bool {
    super::model_discovery::model(base_model(model)).map_or(seed, |entry| {
        entry.service_tiers.iter().any(|t| t.id == tier)
    })
}
/// Selector suffixes supported by this model, in display order.
pub(crate) fn suffixes(model: &str) -> Vec<&'static str> {
    [
        ("-fast", supports_fast(model)),
        ("-ultrafast", supports_ultrafast(model)),
    ]
    .into_iter()
    .filter_map(|(suffix, supported)| supported.then_some(suffix))
    .collect()
}
pub(super) fn parse_fast_alias(model: &str) -> Option<&str> {
    model
        .strip_suffix("-fast")
        .filter(|base| supports_fast(base))
}
fn base_model(model: &str) -> &str {
    let model = model.rsplit('/').next().unwrap_or(model);
    model.split(':').next().unwrap_or(model)
}
#[cfg(test)]
#[path = "service_tier_catalog_tests.rs"]
mod tests;
