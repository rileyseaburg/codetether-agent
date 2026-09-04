//! Runtime tool permission decisions.
//!
//! This module converts effective configuration into a reusable tool
//! invocation decision. It does not prompt users or execute tools.

mod approval;
mod approval_gate;
mod approval_output;
mod approval_prefix;
mod base;
mod code;
mod command;
mod command_prefix;
mod command_rule;
mod command_unsafe;
mod invocation;
mod invocation_decision;
mod invocation_scope;
mod justification;
mod permissions;
mod policy;
mod result;
mod sandbox_preflight;
mod session_command;
mod tool_kind;
mod types;
mod workspace;

pub use approval_gate::{approved_invocation, approved_or_session_command};
pub use command::is_read_only_command;
pub use invocation::{
    evaluate_tool_invocation, evaluate_tool_invocation_for_workspace,
    evaluate_tool_invocation_with_config,
};
pub use policy::RuntimeToolPolicy;
pub use result::{blocking_result, evaluate_tool, evaluate_tool_with_config};
pub use tool_kind::ToolKind;
pub use types::{DecisionReason, ToolPolicyDecision, ToolPolicyOutcome};

#[cfg(test)]
include!("test_modules.rs");
