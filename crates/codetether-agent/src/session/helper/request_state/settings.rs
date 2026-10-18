//! Shared provider capability, temperature and prompt composition.
pub(super) use codetether_request_preparation::{model_supports_tools, system_prompt_for};
#[cfg(test)]
#[path = "settings_tests.rs"]
mod tests;
