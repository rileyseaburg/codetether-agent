//! Shared lifecycle for long-running shell commands.

#[path = "command_session/activity.rs"]
mod activity;
#[path = "command_session/buffer.rs"]
mod buffer;
#[path = "command_session/drain.rs"]
mod drain;
#[path = "command_session/readers.rs"]
mod readers;
#[path = "command_session/register.rs"]
mod register_tools;
#[path = "command_session/registry.rs"]
mod registry;
#[path = "command_session/result.rs"]
mod result;
#[path = "command_session/running.rs"]
mod running;
#[path = "command_session/spawn.rs"]
mod spawn;
#[path = "command_session/types.rs"]
mod types;

pub(crate) use register_tools::register;
pub(crate) use registry::Registry;
pub(crate) use result::tool_result;
pub(crate) use running::Running;
pub(crate) use spawn::command;
pub(crate) use types::{Poll, SpawnMetadata};

#[cfg(test)]
#[path = "command_session/test_modules.rs"]
mod test_modules;