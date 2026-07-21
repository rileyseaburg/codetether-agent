//! Fail-closed lease acquisition immediately before a mutating tool call.

use serde_json::Value;

pub(in crate::session::helper) async fn blocked(
    tool: &str,
    input: &Value,
) -> Option<super::super::tool_policy::ToolTuple> {
    if !crate::mux::coordination::active() {
        return None;
    }
    if let Some(action) = super::delegation::work_action(tool, input) {
        return Some(super::gate_error::delegation(tool, action));
    }
    let paths = super::paths::mutation_paths(tool, input)?;
    let context = match super::context::RuntimeContext::from_input(input) {
        Ok(context) => context,
        Err(error) => return Some(super::gate_failure::unavailable(tool, &error)),
    };
    let scope = super::scope::resolve(&context.workspace, paths);
    if super::scope::too_broad(&scope) {
        return Some(super::scope_error::result(tool, &scope.workspace));
    }
    let reply = crate::mux::coordination::acquire(
        &context.owner,
        &context.agent,
        &scope.workspace,
        scope.paths,
    )
    .await;
    match reply {
        Ok(Some(reply)) => super::outcome::classify(tool, &context.agent, reply),
        Ok(None) => Some(super::gate_failure::missing(tool)),
        Err(error) => Some(super::gate_failure::unavailable(tool, &error)),
    }
}
