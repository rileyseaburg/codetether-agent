//! First-class collaboration lifecycle tools and trusted context forwarding.
//!
//! [RuntimeContext] preserves injected session, workspace and model values.
//! Implement [Host] with an existing agent runtime, then construct the spawn,
//! interrupt, close or resume tool. [execute] forwards legacy payloads after
//! injecting the same trusted context.
mod close;
mod context;
mod host;
mod interrupt;
mod legacy;
mod resume;
mod spawn;
pub use close::CloseAgentTool;
pub use context::RuntimeContext;
pub use host::Host;
pub use interrupt::InterruptAgentTool;
pub use legacy::execute;
pub use resume::ResumeAgentTool;
pub use spawn::SpawnAgentTool;

mod availability;
mod ensure;
mod ensure_public;
mod followup;
mod list;
mod messaging_host;
mod route;
mod send_message;
pub use availability::EnsureOpen;
pub use ensure_public::ensure_ready;
pub use followup::FollowupTaskTool;
pub use list::{ListAgentsTool, ListArgs};
pub use messaging_host::MessagingHost;
pub use route::Route;
pub use send_message::SendMessageTool;

mod send_input;
pub use send_input::{SendInputArgs, SendInputTool};

mod wait;
mod wait_host;
mod wait_public;
pub use codetether_collaboration_state::{Activity, ThreadStatus};
pub use wait::{WaitAgentTool, WaitArgs};
pub use wait_host::{WaitAgent, WaitHost};
pub use wait_public::execute_wait;

mod application_binding;

/// Dependencies used by application-local collaboration host declarations.
#[doc(hidden)]
pub mod __private {
    pub use anyhow::Result;
    pub use async_trait::async_trait;
    pub use codetether_tool_core::{Tool, ToolResult};
    pub use serde_json::{Value, json};
    pub use tokio::{sync::watch, time::Instant};
}

mod catalog_binding;
