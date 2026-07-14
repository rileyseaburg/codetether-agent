//! # Sub-Agent Loop
//!
//! Runs provider turns and requested tools until completion or a safety limit.
//! The private `State` type owns conversation history, counters, and deadlines;
//! [`run_agent_loop`] is the stable entry point used by swarm and Ralph callers.
//!
//! Callers supply a provider, tool registry, prompts, and execution limits. The
//! returned tuple contains accumulated text, steps, tool calls, and exit reason.

mod api;
mod create;
mod driver;
mod limits;
mod output;
mod preview;
mod request;
mod state;
mod thinking;
mod tool_bus;
mod tool_events;
mod tool_input;
mod tool_one;
mod tool_output;
mod tool_raw;
mod tool_raw_result;
mod tools;
mod turn;
pub use api::run_agent_loop;
