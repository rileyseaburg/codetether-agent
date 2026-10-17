//! Raw tool arguments carried on a live approval for proposed-content analysis.

use super::LiveApprovalRequest;

impl LiveApprovalRequest {
    /// Attaches the raw tool arguments so clients can reconstruct the exact
    /// file contents a mutation would produce and run diagnostics on them.
    pub fn with_arguments(mut self, arguments: serde_json::Value) -> Self {
        self.arguments = Some(arguments);
        self
    }
}
