//! Runtime tool permission decisions.
//!
//! This module converts effective configuration into a reusable tool
//! invocation decision. It does not prompt users or execute tools.

mod approval_gate;
mod command;
mod invocation;
mod invocation_scope;
mod justification_schema;
use codetether_runtime_policy as policy;
mod result;
mod sandbox_preflight;
use codetether_runtime_policy as tool_kind;
mod types;

#[doc(hidden)]
pub use approval_gate::approval_binding;
pub use approval_gate::{approved_invocation, approved_or_session_command};
pub use command::is_read_only_command;
pub use invocation::{
    evaluate_tool_invocation, evaluate_tool_invocation_for_workspace,
    evaluate_tool_invocation_with_config,
};
#[doc(hidden)]
pub use justification_schema::apply as require_justification_in_ask_mode;
pub use justification_schema::note_access_mode;
pub use policy::RuntimeToolPolicy;
pub use result::{blocking_result, evaluate_tool, evaluate_tool_with_config};
pub use tool_kind::ToolKind;
pub use types::{DecisionReason, ToolPolicyDecision, ToolPolicyOutcome};

#[cfg(test)]
include!("test_modules.rs");
