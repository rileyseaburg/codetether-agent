//! Shared conversion from prompt completion or panic into a runtime notice.
pub(super) use codetether_session_events::completion::{PromptRunResult, notice};
#[cfg(test)]
#[path = "prompt_result_tests.rs"]
mod tests;
