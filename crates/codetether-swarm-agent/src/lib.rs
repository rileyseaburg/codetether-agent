//! Swarm sub-agent turns and tool execution over application-provided policy.
//!
//! [run_agent_loop] owns turn state, completion limits, tool sequencing and
//! publication. [Host] supplies tool lookup, denial policy and goal accounting.
mod api;
mod completion;
mod control;
#[doc(hidden)]
pub mod driver;
mod executor_trace;
mod host;
mod limits;
mod output;
mod request;
mod runtime;
#[doc(hidden)]
pub mod state;
mod thinking;
mod tool_bus;
mod tool_events;
mod tool_input;
mod tool_one;
mod tool_output;
mod tool_raw;
#[doc(hidden)]
pub mod tools;
pub use api::run_agent_loop;
use codetether_swarm_core::agent_retry::AgentLoopExit;
pub use host::Host;
