//! Server protocol response construction, independent of handler execution.
//!
//! Use initialization with a caller-supplied capability callback, built-in
//! prompts for the standard coding templates, and tool-result normalization
//! before publishing application events. Wire records live in [crate::types].
use crate::types::*;
use serde_json::Value;
mod initialize;
mod prompt_explanation;
mod prompt_review;
mod prompts;
mod tool_result;
pub use initialize::initialize;
pub use prompts::{get_prompt, list_prompts};
pub use tool_result::normalize_tool_result;
