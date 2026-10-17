//! Narrow model, tool, and event interfaces implemented by the host application.

mod events;
mod message;
mod provider;
mod response;
mod rewriter;
mod tool;
pub use events::RlmEventBus;
pub use message::LlmMessage;
pub use provider::LlmProvider;
pub use response::LlmResponse;
pub use rewriter::ToolCallRewriter;
pub use tool::{ToolCall, ToolDefinition};
