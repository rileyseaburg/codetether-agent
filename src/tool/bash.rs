//! Shared bash tool and application approval regression tests.
#[cfg(test)]
use super::Tool;
pub use codetether_bash_tool::BashTool;
#[cfg(test)]
use codetether_bash_tool::{
    codetether_wrapped_command, interactive_auth_risk_reason, looks_like_auth_prompt,
};
#[cfg(test)]
#[path = "bash_tests.rs"]
mod tests;
