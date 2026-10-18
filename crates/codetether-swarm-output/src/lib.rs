//! Bounded tool-output processing for swarm agent conversations.
//!
//! Supply a [Context] and call [render] to truncate ordinary output or summarize
//! oversized output with RLM under the agent deadline. The result retains tool
//! feedback formatting and reports whether summarization exceeded the deadline.
mod agent_loop_large_result;
mod context;
mod large_result;
mod large_result_format;
mod render;
pub use context::Context;
pub use render::render;
