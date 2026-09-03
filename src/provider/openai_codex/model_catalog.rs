//! Models supported by or announced for the ChatGPT-backed Codex provider.
//!
//! Rollout-gated entries can be selected before an account receives access.

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

/// Models understood by CodeTether's ChatGPT Codex backend, in selector order.
/// The upstream service remains authoritative for rollout-gated availability.
///
/// # Returns
///
/// A stable ordered slice shared by provider validation and failover policy.
///
/// # Examples
///
/// ```ignore
/// assert!(chatgpt_models().contains(&"gpt-5.6-sol"));
/// ```
pub(crate) fn chatgpt_models() -> &'static [&'static str] {
    CHATGPT_MODELS
}