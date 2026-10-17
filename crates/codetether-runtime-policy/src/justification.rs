//! Model-supplied justification required before an `ask`-mode approval prompt.
//!
//! In access mode `ask` the user is interrupted for every mutating tool call,
//! so the runtime refuses to raise the prompt until the model states why the
//! action is needed. The justification becomes the stored approval reason and
//! is shown alongside the request in interactive clients.

use super::{ToolKind, ToolPolicyDecision, code};
use codetether_config::{AccessMode, Config};
use codetether_tool_core::ToolResult;
use serde_json::{Value, json};

/// Argument name the model uses to explain a request.
pub const FIELD: &str = "justification";

/// Trimmed, non-empty justification supplied with the invocation.
///
/// # Arguments
/// * `args` — Invocation arguments supplied by the model.
/// # Returns
/// Non-empty trimmed justification, or `None` if absent or blank.
/// # Examples
/// ```
/// let args = serde_json::json!({"justification": "  update the requested file  "});
/// assert_eq!(codetether_runtime_policy::justification::from_args(&args), Some("update the requested file"));
/// ```
pub fn from_args(args: &Value) -> Option<&str> {
    args.get(FIELD)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|text| !text.is_empty())
}

/// Blocking result when `ask` mode needs a justification the model omitted.
///
/// # Arguments
/// * `config` — Configuration supplying the effective access mode.
/// * `tool_name` — Tool identifier used for side-effect classification.
/// * `decision` — Existing decision for diagnostic metadata.
/// * `args` — Invocation arguments.
/// # Returns
/// A structured refusal when justification is required and absent;
/// otherwise `None`. This function never creates an approval request.
/// # Examples
/// ```
/// use codetether_runtime_policy::{justification, RuntimeToolPolicy};
/// let config = codetether_config::Config::default();
/// let decision = RuntimeToolPolicy::from_config(&config).decide_tool("read");
/// assert!(justification::missing(&config, "read", decision, &serde_json::json!({})).is_none());
/// ```
pub fn missing(
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
             Nothing ran. Retry the same call with identical arguments plus a one-sentence \
             `justification` tied to the current request (see `example`). Do not change \
             other arguments, or the request will need a fresh approval.",
            Some(vec![FIELD]),
            Some(super::justification_example::retry(args)),
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
