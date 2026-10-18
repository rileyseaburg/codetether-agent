//! First-class persisted goal tools plus the legacy session-task surface.

#[path = "goal/attempt_cap.rs"]
pub mod attempt_cap;
#[path = "goal/budget_guard.rs"]
pub(crate) mod budget_guard;
#[path = "goal/context.rs"]
mod context;
#[path = "goal/create.rs"]
mod create;
#[path = "goal/create_run.rs"]
mod create_run;
#[path = "goal/create_validate.rs"]
mod create_validate;
#[path = "goal/edit.rs"]
mod edit;
#[path = "goal/edit_run.rs"]
mod edit_run;
#[path = "goal/edit_schema.rs"]
mod edit_schema;
#[cfg(test)]
#[path = "goal/edit_tests.rs"]
mod edit_tests;
#[path = "goal/get.rs"]
mod get;
#[path = "goal/response.rs"]
mod response;
#[path = "session_task/mod.rs"]
mod session_task;
#[path = "goal/update.rs"]
mod update;
#[path = "goal/update_gate.rs"]
mod update_gate;
#[path = "goal/update_run.rs"]
mod update_run;
#[path = "goal/verdict_log.rs"]
pub mod verdict_log;
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
