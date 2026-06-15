//! Ralph handoff/progress publish helpers for [`BusHandle`] (split for budget).

use super::{BusHandle, BusMessage};

impl BusHandle {
    /// Publish a context handoff between sequential Ralph stories.
    pub fn publish_ralph_handoff(
        &self,
        prd_id: &str,
        from_story: &str,
        to_story: &str,
        context: serde_json::Value,
        progress_summary: &str,
    ) -> usize {
        self.send(
            format!("ralph.{prd_id}"),
            BusMessage::RalphHandoff {
                prd_id: prd_id.to_string(),
                from_story: from_story.to_string(),
                to_story: to_story.to_string(),
                context,
                progress_summary: progress_summary.to_string(),
            },
        )
    }

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
