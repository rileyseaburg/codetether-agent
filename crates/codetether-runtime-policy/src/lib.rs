//! Reusable runtime permission decisions derived from effective configuration.
//!
//! Build [RuntimeToolPolicy] from configuration, then classify tools and actions.
//! Approval records and policy evaluation share reusable owners; application
//! callers supply prior-context guards and OS sandbox availability probes.
mod action_read_only;
mod base;
pub mod code;
pub mod command;
mod command_prefix;
mod command_rule;
mod command_unsafe;
mod permissions;
mod policy;
mod policy_action;
mod tool_kind;
mod types;
pub use policy::RuntimeToolPolicy;
pub use tool_kind::ToolKind;
pub use types::{DecisionReason, ToolPolicyDecision, ToolPolicyOutcome};

pub mod blocking;
pub mod invocation;
mod invocation_rules;
pub mod justification;
mod justification_example;
pub mod sandbox;

pub mod approval_output;
pub mod justification_schema;
pub mod result_approval_args;

pub mod approval;
pub mod approval_gate;
pub mod invocation_scope;
pub mod result_approval;
pub mod session_command;

pub mod evaluate;
pub mod evaluate_tool;
pub mod workspace;
