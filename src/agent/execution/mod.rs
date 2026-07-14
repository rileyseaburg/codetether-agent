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

mod image_inject;
mod messages;
mod model;
mod request;
mod run;
mod tool_calls;
mod tool_calls_parallel;
mod tool_calls_policy;
mod tool_input;
mod tool_result_record;

#[cfg(test)]
mod tool_calls_policy_tests;
#[cfg(test)]
mod tool_input_tests;
