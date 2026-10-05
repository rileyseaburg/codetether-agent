//! First-class persisted goal tools plus the legacy session-task surface.

#[path = "goal/attempt_cap.rs"]
pub mod attempt_cap;
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
#[path = "goal/update_escalate.rs"]
mod update_escalate;
#[path = "goal/update_gate.rs"]
mod update_gate;
#[path = "goal/update_reject.rs"]
mod update_reject;
#[path = "goal/update_run.rs"]
mod update_run;
#[path = "goal/verdict_log.rs"]
pub mod verdict_log;
#[path = "goal/verify.rs"]
pub mod verify;

use super::ToolRegistry;
use std::sync::Arc;

/// Register goal lifecycle tools and the backward-compatible task tool.
pub fn register(registry: &mut ToolRegistry) {
    registry.register(Arc::new(session_task::SessionTaskTool::new()));
    registry.register(Arc::new(get::GetGoalTool));
    registry.register(Arc::new(create::CreateGoalTool));
    registry.register(Arc::new(update::UpdateGoalTool));
    registry.register(Arc::new(edit::EditGoalTool));
}
