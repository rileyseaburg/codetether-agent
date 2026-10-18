//! Agent state ownership with native tool and session bindings.
//!
//! [Agent] retains the provider, tool registry and permission metadata while
//! dedicated completion, replay and tool crates own execution policy.
//! [Host] connects native registries and sessions; swarm traits are implemented
//! on the same state owner.
mod core;
mod execution;
mod host;
mod swarm;
mod tooling;
pub use codetether_agent_core::{AgentInfo, AgentResponse, ToolMetadata, ToolUse};
pub use core::Agent;
pub use host::{Context, Host, Registry, SessionView, SessionWrite};
