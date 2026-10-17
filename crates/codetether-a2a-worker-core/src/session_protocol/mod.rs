//! Worker provider-response and step-budget protocol helpers.
//!
//! [collect_tool_calls] and [append_text_output] preserve provider response
//! ordering. [wrap_up_step] and [wrap_up_message] implement the budget notice.
//! [SessionStepsOutcome] carries the final text and budget-exhaustion status.
mod collect;
mod outcome;
mod output;
mod wrap_up;
mod wrap_up_step;
pub use collect::{ToolCall, collect_tool_calls};
pub use outcome::SessionStepsOutcome;
pub use output::append_text_output;
pub use wrap_up::wrap_up_message;
pub use wrap_up_step::wrap_up_step;
#[cfg(test)]
mod wrap_up_tests;

/// Host-driven bounded worker execution loop.
pub mod driver;
