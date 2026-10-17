//! Serializable profile, tool, and result contracts shared by agent consumers.
//!
//! [AgentInfo] selects behavior; [ToolMetadata] describes available tools;
//! [AgentResponse] records the result and [ToolUse] records executed calls.
mod profile;
mod response;
mod tool;
pub use profile::{AgentInfo, AgentMode};
pub use response::AgentResponse;
pub use tool::{ToolMetadata, ToolUse};
