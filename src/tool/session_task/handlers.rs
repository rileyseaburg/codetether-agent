//! Per-action handlers for the `session_task` tool.

#[path = "handlers/goal.rs"]
mod goal;
#[path = "handlers/list.rs"]
mod list_handler;
#[path = "handlers/status_parse.rs"]
mod status_parse;
#[path = "handlers/task.rs"]
mod task;

#[path = "handlers/set.rs"]
mod set_handler;
pub(super) use goal::{clear_goal, reaffirm};
pub(super) use list_handler::list;
pub(super) use set_handler::set_goal;
pub(super) use task::{task_add, task_status};
#[path = "handlers/complete.rs"]
mod complete;
pub(super) use complete::complete_goal;
#[path = "handlers/clear_guard.rs"]
mod clear_guard;
#[cfg(test)]
#[path = "handlers/clear_guard_tests.rs"]
mod clear_guard_tests;
#[path = "handlers/set_guard.rs"]
mod set_guard;
#[cfg(test)]
#[path = "handlers/set_guard_tests.rs"]
mod set_guard_tests;
