//! Shared provider-turn sequencing and response interpretation for sub-agents.
//!
//! [execute] runs a host [AgentRuntime]; [parse_turn] separates provider response
//! text, thinking and tool calls. [after_turn] chooses execution or completion.
use crate::agent_retry::AgentLoopExit;
use anyhow::Result;
use codetether_provider_core::{CompletionResponse, ContentPart, FinishReason, Message};
use std::time::Instant;
mod types;
pub use types::{RequestOutcome, ToolCall, Turn};
mod turn;
pub use turn::parse_turn;
mod step;
pub use step::{AfterTurn, after_turn};
mod limits;
pub use limits::limit_reached;
mod advance;
pub use advance::advance_turn;
mod contract;
pub use contract::AgentRuntime;
mod driver;
pub use driver::execute;

/// Live sub-agent event publication and output accumulation.
pub mod events;

/// Sequential tool batches and result-message construction.
pub mod tools;
