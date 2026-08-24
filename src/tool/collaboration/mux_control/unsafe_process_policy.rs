//! Explicit authority for mux lifecycle actions that launch a direct PTY child.

use super::args::Args;
use crate::tool::ToolResult;
use serde_json::Value;

pub(super) async fn blocked(args: &Args, input: &Value) -> Option<ToolResult> {
    if let Some(blocked) = crate::runtime_policy::evaluate_tool_invocation("mux_control", input).await {
        return Some(blocked);
    }
    if !launches_process(&args.action) {
        return None;
    }
    let allowed = authorized(
        crate::tool::sandbox::direct_fallback_env_allowed(),
        crate::runtime_policy::approved_receipt("mux_control", input),
    );
    (!allowed).then(|| ToolResult::structured_error(
        "UNSAFE_FALLBACK_REQUIRED", "mux_control",
        "Mux start/roll requires an exact approval and the explicit unsafe fallback setting.",
        None, None,
    ))
}

fn launches_process(action: &str) -> bool { matches!(action, "start" | "roll") }

fn authorized(unsafe_fallback: bool, exact_approval: bool) -> bool {
    unsafe_fallback && exact_approval
}

#[cfg(test)]
#[path = "unsafe_process_policy_tests.rs"]
mod tests;