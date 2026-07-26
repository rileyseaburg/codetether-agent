//! Opt-in lightweight system prompt selection for local models.
//!
//! Small locally served models cannot afford the full agent system prompt.
//! On CPU-only hosts a 32k-token prompt costs minutes of prefill per turn,
//! so this module decides when to substitute the compact prompt.

/// Return whether the compact local prompt should replace the full one.
///
/// The compact prompt is used for `local_cuda` providers, or for any
/// provider when `CODETETHER_LIGHT_SYSTEM_PROMPT` is set to `1` or `true`.
///
/// # Arguments
///
/// * `provider` — Selected provider identifier.
///
/// # Returns
///
/// `true` when the caller should use the compact prompt.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::helper::runtime::prefers_light_system_prompt;
///
/// assert!(prefers_light_system_prompt("local_cuda"));
/// assert!(!prefers_light_system_prompt("openai"));
/// ```
pub fn prefers_light_system_prompt(provider: &str) -> bool {
    super::prompt::is_local_cuda_provider(provider) || light_prompt_forced()
}

fn light_prompt_forced() -> bool {
    std::env::var("CODETETHER_LIGHT_SYSTEM_PROMPT")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}
