//! publish ralph progress operations for the shared bus runtime.
use super::*;
impl BusHandle {
    /// Publish PRD-level progress.
    pub fn publish_ralph_progress(
        &self,
        prd_id: &str,
        passed: usize,
        total: usize,
        iteration: usize,
        status: &str,
    ) -> usize {
        self.send(
            format!("ralph.{prd_id}"),
            BusMessage::RalphProgress {
                prd_id: prd_id.to_string(),
                passed,
                total,
                iteration,
                status: status.to_string(),
            },
        )
    }
}
