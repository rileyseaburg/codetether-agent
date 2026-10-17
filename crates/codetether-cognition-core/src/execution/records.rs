//! Tool invocation outcomes and auditable decision receipts.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A single tool invocation record.
///
/// # Examples
/// ```
/// use codetether_cognition_core::execution::ToolInvocation;
/// let record = ToolInvocation { tool_name: "read".into(), arguments: serde_json::json!({}),
///     result: None, success: false, lease_id: "lease-1".into(), invoked_at: chrono::Utc::now() };
/// assert_eq!(record.lease_id, "lease-1");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInvocation {
    pub tool_name: String,
    pub arguments: Value,
    pub result: Option<String>,
    pub success: bool,
    pub lease_id: String,
    pub invoked_at: DateTime<Utc>,
}

/// Outcome of a proposal execution.
///
/// Variants distinguish Success from Failure with optional follow-up attention.
///
/// # Examples
/// ```
/// use codetether_cognition_core::execution::ExecutionOutcome;
/// match (ExecutionOutcome::Success { summary: "done".into() }) {
///     ExecutionOutcome::Success { summary } => assert_eq!(summary, "done"),
///     ExecutionOutcome::Failure { .. } => unreachable!(),
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ExecutionOutcome {
    Success {
        summary: String,
    },
    Failure {
        error: String,
        follow_up_attention: Option<String>,
    },
}

/// Full audit trail for a proposal execution.
///
/// # Examples
/// ```
/// use codetether_cognition_core::execution::{DecisionReceipt, ExecutionOutcome};
/// let receipt = DecisionReceipt { id: "r1".into(), proposal_id: "p1".into(), inputs: vec![],
///     governance_decision: "approved".into(), capability_leases: vec![], tool_invocations: vec![],
///     outcome: ExecutionOutcome::Success { summary: "done".into() }, created_at: chrono::Utc::now() };
/// assert_eq!(receipt.proposal_id, "p1");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionReceipt {
    pub id: String,
    pub proposal_id: String,
    pub inputs: Vec<String>,
    pub governance_decision: String,
    pub capability_leases: Vec<String>,
    pub tool_invocations: Vec<ToolInvocation>,
    pub outcome: ExecutionOutcome,
    pub created_at: DateTime<Utc>,
}
