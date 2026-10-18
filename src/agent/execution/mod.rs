//! Agent execution workflow.
//!
//! This module owns the prompt loop, completion request construction, and
//! response extraction used by `Agent::execute`.
//!
//! # Examples
//!
//! ```ignore
//! let response = agent.execute(&mut session, "fix it").await?;
//! ```

mod completion_host;
mod messages;
mod replay_host;
mod run;
#[cfg(test)]
mod tool_calls_policy;
mod tool_image_context;
mod tool_input;
#[cfg(test)]
mod tool_result_record;

#[cfg(test)]
mod tool_calls_policy_tests;
#[cfg(test)]
mod tool_image_context_tests;
#[cfg(test)]
mod tool_input_tests;
#[cfg(test)]
mod tool_result_record_tests;
