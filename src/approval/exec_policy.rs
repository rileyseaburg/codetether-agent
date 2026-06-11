use serde::{Deserialize, Serialize};

/// Proposed exec-policy change for commands with a shared prefix.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct ExecPolicyAmendment {
    pub command: Vec<String>,
}

impl ExecPolicyAmendment {
    /// Create a proposed command-prefix amendment.
    pub fn new(command: Vec<String>) -> Self {
        Self { command }
    }

    /// Return the command tokens that make up the reusable prefix.
    pub fn command(&self) -> &[String] {
        &self.command
    }

    pub(crate) fn prefix_string(&self) -> Option<String> {
        let tokens = self
            .command
            .iter()
            .map(|token| token.trim())
            .filter(|token| !token.is_empty())
            .collect::<Vec<_>>();
        (!tokens.is_empty()).then(|| tokens.join(" "))
    }
}

/// User decision returned for an approval request.
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
