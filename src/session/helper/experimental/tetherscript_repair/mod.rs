//! Post-prune repair for DeepSeek V4 reasoning_content via tetherscript.
//!
//! After thinking_prune strips old Thinking blocks, assistant messages
//! with tool_calls lose their reasoning_content. DeepSeek V4 requires
//! every assistant with tool_calls to carry a non-null
//! `reasoning_content` field.
//!
//! Calls the bundled `deepseek_repair.tether` hook per-message.

mod convert;
#[cfg(not(feature = "tetherscript"))]
mod disabled;
#[cfg(feature = "tetherscript")]
mod enabled;

use super::ExperimentalStats;
use crate::provider::Message;

/// Run the tetherscript repair on messages after thinking_prune.
pub fn repair_reasoning(messages: &mut Vec<Message>) -> ExperimentalStats {
    #[cfg(feature = "tetherscript")]
    return enabled::repair_reasoning(messages);
    #[cfg(not(feature = "tetherscript"))]
    {
        disabled::repair_reasoning(messages)
    }
}
