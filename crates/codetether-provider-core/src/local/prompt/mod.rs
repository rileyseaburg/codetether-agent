//! Prompt projection and finish-reason normalization for local adapters.
use crate::{ContentPart, FinishReason, Message, Role};
/// Stateless local-provider prompt formatting.
///
/// # Examples
/// ```
/// let text = codetether_provider_core::local::LocalCudaPrompt::strip_inline_think_blocks("<think>draft</think>answer");
/// assert_eq!(text, "answer");
/// ```
pub struct LocalCudaPrompt;
mod content_to_string;
mod format_messages;
mod map_finish_reason;
mod strip_inline_think_blocks;
mod to_prompts;
