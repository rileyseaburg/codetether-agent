//! Built-in prompt constants.
//!
//! This module groups the raw prompt templates used by built-in agents.
//!
//! # Examples
//!
//! ```ignore
//! assert!(BUILD_SYSTEM_PROMPT.contains("CodeTether Agent"));
//! ```

mod build;
mod explore;
mod plan;

pub use build::{BUILD_MODE_GUARDRAIL, BUILD_SYSTEM_PROMPT};
pub use explore::EXPLORE_SYSTEM_PROMPT;
pub use plan::PLAN_SYSTEM_PROMPT;
