//! Mux-authoritative shared-worktree mutation coordination.

#[path = "workspace_coordination/batch.rs"]
mod batch;
#[path = "workspace_coordination/command_path.rs"]
mod command_path;
#[path = "workspace_coordination/context.rs"]
mod context;
#[path = "workspace_coordination/delegation.rs"]
mod delegation;
#[path = "workspace_coordination/gate.rs"]
mod gate;
#[path = "workspace_coordination/gate_error.rs"]
mod gate_error;
#[path = "workspace_coordination/gate_failure.rs"]
mod gate_failure;
#[path = "workspace_coordination/outcome.rs"]
mod outcome;
#[path = "workspace_coordination/patch.rs"]
mod patch;
#[path = "workspace_coordination/paths.rs"]
mod paths;
#[path = "workspace_coordination/scope.rs"]
mod scope;
#[path = "workspace_coordination/scope_error.rs"]
mod scope_error;
#[path = "workspace_coordination/shell.rs"]
mod shell;
#[path = "workspace_coordination/structured.rs"]
mod structured;
#[path = "workspace_coordination/turn.rs"]
mod turn;

pub(super) use gate::blocked;
pub(super) use turn::LeaseTurn;

#[cfg(test)]
mod command_path_tests;
#[cfg(test)]
mod delegation_tests;
#[cfg(test)]
mod patch_tests;
#[cfg(test)]
mod scope_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tool_tests;
