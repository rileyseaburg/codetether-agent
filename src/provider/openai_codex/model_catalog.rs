//! Offline seed catalog for the ChatGPT-backed Codex provider.
//!
//! Live discovery ([`super::model_discovery`]) is authoritative. This list is
//! only used before the first successful `/models` query in a process, or
//! when the account is not authenticated, so pickers and failover still have
//! candidates.

#[path = "model_catalog_live.rs"]
mod live;

const CHATGPT_MODELS: &[&str] = &[
    "gpt-6-astra",
    "gpt-6-astra-fast",
    "gpt-5.5",
    "gpt-5.5-fast",
    "gpt-5.6-sol",
    "gpt-5.6-terra",
    "gpt-5.6-luna",
    "gpt-reserve",
    "gpt-5.4",
    "gpt-5.4-mini",
    "codex-auto-review",
];

/// Offline seed models, in selector order.
///
/// # Returns
///
/// A stable ordered slice used when no discovery has succeeded yet.
///
/// # Examples
///
/// ```ignore
/// assert!(chatgpt_models().contains(&"gpt-5.6-sol"));
/// ```
pub(crate) fn chatgpt_models() -> &'static [&'static str] {
    CHATGPT_MODELS
}

/// Models for failover and routing: the latest live discovery when one has
/// succeeded in this process, otherwise the offline seed.
pub(crate) fn current_models() -> &'static [&'static str] {
    live::current().unwrap_or(CHATGPT_MODELS)
}
