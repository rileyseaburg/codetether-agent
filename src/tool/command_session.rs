//! Application tool registration around shared command sessions.
pub(crate) use codetether_command_session::{Poll, Registry, Running, SpawnMetadata, tool_result};

pub(crate) use codetether_application_registry::commands::register;

pub(crate) use codetether_command_session::spawn as command;

#[cfg(test)]
#[path = "command_session/buffer_integration_tests.rs"]
mod buffer_integration_tests;
#[cfg(test)]
#[path = "command_session/escalation_tests.rs"]
mod escalation_tests;
#[cfg(test)]
#[path = "command_session/liveness_tests.rs"]
mod liveness_tests;
#[cfg(all(test, unix))]
#[path = "command_session/pty_tests.rs"]
mod pty_tests;
#[cfg(all(test, unix))]
#[path = "command_session/sandbox_tests.rs"]
mod sandbox_tests;
#[cfg(test)]
#[path = "command_session/tests.rs"]
mod tests;
#[cfg(test)]
#[path = "command_session/tool_tests.rs"]
mod tool_tests;
