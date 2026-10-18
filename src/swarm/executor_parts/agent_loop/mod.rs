//! # Sub-Agent Loop
//!
//! Runs provider turns and requested tools until completion or a safety limit.
//! The private `State` type owns conversation history, counters, and deadlines;
//! [`run_agent_loop`] is the stable entry point used by swarm and Ralph callers.
//!
//! Callers supply a provider, tool registry, prompts, and execution limits. The
//! returned tuple contains accumulated text, steps, tool calls, and exit reason.

mod api;
mod completion;
mod control;
mod driver;
mod limits;
mod output;
mod request;
mod runtime;
mod state;
mod thinking;
mod tool_bus;
mod tool_events;
mod tool_input;
mod tool_one;
mod tool_output;
mod tool_raw;
mod tools;
pub use api::run_agent_loop;

#[cfg(test)]
mod control_goal_tests;
#[cfg(test)]
mod control_test_provider;
#[cfg(test)]
mod control_test_response;
#[cfg(test)]
mod control_tests;
#[cfg(test)]
mod image_test_provider;
#[cfg(test)]
mod image_test_state;
#[cfg(test)]
mod image_test_tool;
#[cfg(test)]
mod image_tests;
