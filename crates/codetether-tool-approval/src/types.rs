//! Ready and blocked outcomes from the tool approval gate.

use serde_json::Value;

use codetether_tool_reporting::ToolTuple;

/// Outcome of preflight, policy and optional interactive approval.
///
/// # Examples
/// ```
/// use codetether_tool_approval::ApprovalGate;
/// let gate = ApprovalGate::Ready(serde_json::json!({}), Vec::new());
/// match gate {
///     ApprovalGate::Ready(_, warnings) => assert!(warnings.is_empty()),
///     ApprovalGate::Blocked(_, _, _) => panic!("unexpected block"),
/// }
/// ```
pub enum ApprovalGate {
    /// Invocation may proceed with these arguments and verification warnings.
    Ready(Value, Vec<String>),
    /// Invocation is blocked; report the tool result and verification warnings.
    Blocked(Value, ToolTuple, Vec<String>),
}

impl ApprovalGate {
    /// Consume the outcome, omitting verification warnings.
    pub fn into_parts(self) -> (Value, Option<ToolTuple>) {
        let (args, blocked, _) = self.into_checked_parts();
        (args, blocked)
    }

    /// Consume the outcome while retaining arguments, a block and warnings.
    pub fn into_checked_parts(self) -> (Value, Option<ToolTuple>, Vec<String>) {
        match self {
            Self::Ready(args, warnings) => (args, None, warnings),
            Self::Blocked(args, tuple, warnings) => (args, Some(tuple), warnings),
        }
    }
}
