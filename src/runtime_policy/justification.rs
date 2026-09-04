//! Model-supplied justification required before an `ask`-mode approval prompt.
//!
//! In access mode `ask` the user is interrupted for every mutating tool call,
//! so the runtime refuses to raise the prompt until the model states why the
//! action is needed. The justification becomes the stored approval reason and
//! is shown alongside the request in interactive clients.

use super::{ToolKind, ToolPolicyDecision, code};
use crate::config::{AccessMode, Config};
use crate::tool::ToolResult;
use serde_json::{Value, json};

/// Argument name the model uses to explain a request.
pub(crate) const FIELD: &str = "justification";

/// Trimmed, non-empty justification supplied with the invocation.
pub(super) fn from_args(args: &Value) -> Option<&str> {
    args.get(FIELD)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|text| !text.is_empty())
}

/// Blocking result when `ask` mode needs a justification the model omitted.
///
/// Returns `None` when no justification is required or one was supplied.
pub(super) fn missing(
    config: &Config,
    tool_name: &str,
    decision: ToolPolicyDecision,
    args: &Value,
) -> Option<ToolResult> {
    if !required(config, tool_name) || from_args(args).is_some() {
        return None;
    }
    Some(
        ToolResult::structured_error(
            "TOOL_JUSTIFICATION_REQUIRED",
            tool_name,
            "Access mode `ask` requires a justification before the user is prompted. \
             Retry with a `justification` stating why this action is needed for the current request.",
            Some(vec![FIELD]),
            Some(json!({ "justification": "<why this action is needed>" })),
        )
        .with_metadata("policy_outcome", json!(code::outcome(decision.outcome)))
        .with_metadata("policy_reason", json!(code::reason(decision.reason)))
        .with_metadata("justification_required", json!(true)),
    )
}

fn required(config: &Config, tool_name: &str) -> bool {
    config.effective_access_mode() == Some(AccessMode::Ask)
        && ToolKind::for_name(tool_name) == ToolKind::Mutating
}
