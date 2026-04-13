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

mod messages;
mod request;
mod run;
mod tool_calls;
