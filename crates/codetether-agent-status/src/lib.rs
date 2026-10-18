//! Owner-scoped child status reporting from the existing bus history.
//!
//! [Host] supplies live child summaries and task ownership. [report] keeps the
//! newest owned task state, supplements it with newer tool activity, classifies
//! liveness and renders the existing status result without executing agents.
mod agent_view;
mod host;
mod report;
mod status_filter;
mod status_liveness;
mod status_source;
mod status_tool_activity;
mod status_tool_filter;
mod tool_events;
mod tool_merge;
pub use agent_view::AgentView;
pub use host::Host;
pub use report::handle_status as report;
mod row;
#[cfg(test)]
mod status_liveness_tests;

pub mod publish;
mod publish_host;
mod task_identity;
mod terminal_messages;
pub use publish_host::{PublishHost, PublishedAgent};
pub use task_identity::{task_id, task_to_agent};
pub use terminal_messages::{result_message, terminal_summary};

mod application_binding;

/// Dependency used by application status declarations.
#[doc(hidden)]
pub mod __private {
    pub use codetether_tool_core::ToolResult;
}
