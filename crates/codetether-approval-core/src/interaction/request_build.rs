//! Request construction and ordered review choices.

use super::LiveApprovalRequest;
use super::{ExecPolicyAmendment, ReviewDecision};

impl LiveApprovalRequest {
    /// Construct a request with approve-once, approve-session and abort choices.
    ///
    /// # Arguments
    /// The identifiers bind the approval to its tool call, tool, action and resource;
    /// `reason` explains the request to the reviewer.
    /// # Returns
    /// A request with optional review details unset.
    /// # Examples
    /// See [LiveApprovalRequest] for a complete construction example.
    pub fn new(
        approval_id: String,
        tool_call_id: String,
        tool: String,
        action: String,
        resource: String,
        reason: String,
    ) -> Self {
        Self {
            approval_id,
            tool_call_id,
            tool,
            action,
            resource,
            reason,
            justification: None,
            preview: None,
            arguments: None,
            proposed_execpolicy_amendment: None,
            available_decisions: default_decisions(None),
        }
    }

    /// Attaches the model's justification for requesting this action.
    pub fn with_justification(mut self, justification: String) -> Self {
        self.justification = Some(justification);
        self
    }

    /// Attaches the full invocation detail for interactive review.
    pub fn with_preview(mut self, preview: String) -> Self {
        self.preview = Some(preview);
        self
    }

    /// Attach an amendment and expose its approval choice before abort.
    pub fn with_execpolicy_amendment(mut self, amendment: ExecPolicyAmendment) -> Self {
        self.available_decisions = default_decisions(Some(&amendment));
        self.proposed_execpolicy_amendment = Some(amendment);
        self
    }
}

fn default_decisions(amendment: Option<&ExecPolicyAmendment>) -> Vec<ReviewDecision> {
    let mut decisions = vec![ReviewDecision::Approved, ReviewDecision::ApprovedForSession];
    if let Some(value) = amendment {
        decisions.push(ReviewDecision::ApprovedExecpolicyAmendment {
            proposed_execpolicy_amendment: value.clone(),
        });
    }
    decisions.push(ReviewDecision::Abort);
    decisions
}
