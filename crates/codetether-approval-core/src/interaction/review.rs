//! Serializable choices an approval client can present to its user.

use super::ExecPolicyAmendment;
use serde::{Deserialize, Serialize};

/// User decision returned for an approval request.
///
/// Variants authorize one request, authorize a session or command amendment,
/// deny, time out, or abort the request. These choices do not apply grants.
///
/// # Examples
/// ```
/// use codetether_approval_core::interaction::ReviewDecision;
/// assert!(matches!(ReviewDecision::default(), ReviewDecision::Denied));
/// assert_eq!(ReviewDecision::Abort.to_opaque_string(), "abort");
/// ```
#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReviewDecision {
    Approved,
    ApprovedExecpolicyAmendment {
        proposed_execpolicy_amendment: ExecPolicyAmendment,
    },
    ApprovedForSession,
    #[default]
    Denied,
    TimedOut,
    Abort,
}

impl ReviewDecision {
    /// Return a stable non-PII decision label for logs and events.
    pub fn to_opaque_string(&self) -> &'static str {
        match self {
            Self::Approved => "approved",
            Self::ApprovedExecpolicyAmendment { .. } => "approved_with_amendment",
            Self::ApprovedForSession => "approved_for_session",
            Self::Denied => "denied",
            Self::TimedOut => "timed_out",
            Self::Abort => "abort",
        }
    }
}
