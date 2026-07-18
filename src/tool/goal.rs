//! First-class persisted goal tools plus the legacy session-task surface.

#[path = "goal/context.rs"]
mod context;
#[path = "goal/create.rs"]
mod create;
#[path = "goal/create_run.rs"]
mod create_run;
#[path = "goal/create_validate.rs"]
mod create_validate;
#[path = "goal/get.rs"]
mod get;
#[path = "goal/response.rs"]
mod response;
#[path = "session_task/mod.rs"]
mod session_task;
#[path = "goal/update.rs"]
mod update;
#[path = "goal/update_run.rs"]
mod update_run;

use super::ToolRegistry;
use std::sync::Arc;

/// Register goal lifecycle tools and the backward-compatible task tool.
pub fn register(registry: &mut ToolRegistry) {
    registry.register(Arc::new(session_task::SessionTaskTool::new()));
    registry.register(Arc::new(get::GetGoalTool));
    registry.register(Arc::new(create::CreateGoalTool));
    registry.register(Arc::new(update::UpdateGoalTool));
}
