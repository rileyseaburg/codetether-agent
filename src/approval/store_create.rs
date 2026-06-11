use super::{ApprovalEvent, ApprovalRequest, ApprovalStore};
use anyhow::Result;

impl ApprovalStore {
    /// Create and persist a pending approval request.
    ///
    /// # Errors
    ///
    /// Returns an error if the request event cannot be appended.
    pub fn create_request(
        &self,
        tool: &str,
        action: &str,
        resource: &str,
        reason: &str,
    ) -> Result<ApprovalRequest> {
        let request = ApprovalRequest::new(tool, action, resource, reason);
        self.append_event(ApprovalEvent::from(request.clone()))?;
        Ok(request)
    }
}
