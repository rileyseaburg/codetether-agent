//! Policy enforcement for synchronous inbound MCP tool handlers.
//!
//! [call_registered] evaluates a registered tool before executing it; [blocked]
//! checks fallback tools using their canonical policy name. [Evaluator] supplies
//! the application's session-aware runtime policy.
mod call;
mod evaluator;
mod result;
pub use call::{blocked, call_registered};
pub use evaluator::Evaluator;
