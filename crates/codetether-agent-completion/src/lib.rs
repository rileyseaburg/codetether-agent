//! Agent completion requests and the model/tool execution loop.
//!
//! [execute] appends the human prompt, preserves completion and tool-result
//! ordering and stops on the first response without tool calls. [Host] binds
//! native session context while the replay crate owns tool-call batching.
mod host;
mod input;
mod model;
mod request;
mod run;
use codetether_agent_core::{AgentInfo, AgentResponse, ToolUse};
use codetether_provider_core::{
    CompletionResponse, ContentPart, Message, Provider, Role, ToolDefinition, Usage,
};
pub use host::{Completion, Host, State};
pub use input::Request;
pub use run::execute;
use std::sync::Arc;
