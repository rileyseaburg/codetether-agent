//! Shared worker model defaults and application temperature policy.
pub(super) use codetether_a2a_worker_core::execution_policy::default_model_for_provider;

/// Reports whether the selected model should run at `temperature = 1.0`.
///
/// Kimi, GLM, and MiniMax model families work best with the fixed temperature
/// policy used elsewhere in the agent stack.
///
/// # Examples
///
/// ```ignore
/// assert!(prefers_temperature_one("zai/glm-5.1"));
/// ```
pub(super) fn prefers_temperature_one(model: &str) -> bool {
    crate::session::helper::provider::prefers_temperature_one(model)
}
