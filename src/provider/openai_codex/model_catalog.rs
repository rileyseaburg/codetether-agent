//! Models supported by the ChatGPT-backed Codex provider.

const CHATGPT_MODELS: &[&str] = &[
    "gpt-5.5",
    "gpt-5.5-fast",
    "gpt-5.6-sol",
    "gpt-5.6-terra",
    "gpt-5.6-luna",
];

/// Models accepted by the ChatGPT Codex backend, in fallback order.
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
