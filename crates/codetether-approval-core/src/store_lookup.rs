use super::{ApprovalDecision, ApprovalEvent, ApprovalRequest, ApprovalStore};
use anyhow::Result;

impl ApprovalStore {
    /// Load a request by approval id.
    ///
    /// # Errors
    ///
    /// Returns an error if the JSONL log cannot be read or decoded.
    pub fn request(&self, approval_id: &str) -> Result<Option<ApprovalRequest>> {
        Ok(self.events()?.into_iter().find_map(|event| match event {
            ApprovalEvent::Request { request } if request.id == approval_id => Some(request),
            ApprovalEvent::Request { .. } | ApprovalEvent::Decision { .. } => None,
        }))
    }

    /// Load the latest decision for an approval id.
    ///
    /// # Errors
    ///
    /// Returns an error if the JSONL log cannot be read or decoded.
    pub fn decision(&self, approval_id: &str) -> Result<Option<ApprovalDecision>> {
        let decision = self
            .events()?
            .into_iter()
            .fold(None, |found, event| match event {
                ApprovalEvent::Decision { decision } if decision.request_id == approval_id => {
                    Some(decision)
                }
                ApprovalEvent::Request { .. } | ApprovalEvent::Decision { .. } => found,
            });
        Ok(decision)
    }
}
