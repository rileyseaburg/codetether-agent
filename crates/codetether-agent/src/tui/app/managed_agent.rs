//! TUI adapter for the canonical managed-agent tool.

#[path = "managed_agent_chat.rs"]
pub(crate) mod chat;
#[path = "managed_agent_command.rs"]
pub(crate) mod command;
#[path = "managed_agent_request.rs"]
mod request;
#[path = "managed_agent_ui.rs"]
pub(crate) mod ui;

pub(crate) use request::{kill, message, spawn};

pub(crate) fn owned(parent_session_id: &str, name: &str) -> bool {
    crate::tool::agent::bridge::find_agent_tool_agent_for_parent(name, parent_session_id).is_some()
}
