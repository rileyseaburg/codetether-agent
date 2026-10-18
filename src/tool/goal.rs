//! First-class persisted goal tools plus the legacy session-task surface.

#[path = "goal/attempt_cap.rs"]
pub mod attempt_cap;
#[path = "goal/create.rs"]
mod create;
#[path = "goal/edit.rs"]
mod edit;
#[path = "goal/get.rs"]
mod get;
#[path = "goal/host.rs"]
mod host;
#[path = "session_task/mod.rs"]
mod session_task;
#[path = "goal/update.rs"]
mod update;
pub use codetether_native_goal_tools::verdict_log;
#[path = "goal/verify.rs"]
pub mod verify;

use super::ToolRegistry;
use codetether_tool_registry::catalog::ToolKind;
#[path = "goal/catalog_construct.rs"]
mod catalog_construct;
pub(crate) use catalog_construct::construct;

/// Register goal lifecycle tools and the backward-compatible task tool.
pub fn register(registry: &mut ToolRegistry) {
    super::catalog::register_group(registry, ToolKind::GoalGroup);
}
