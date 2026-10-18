//! Shared bash tool and application approval regression tests.
#[cfg(test)]
use super::Tool;
pub use crate::BashTool;
#[cfg(test)]
use crate::{codetether_wrapped_command, interactive_auth_risk_reason, looks_like_auth_prompt};
#[cfg(test)]
#[path = "bash_tests.rs"]
mod tests;
