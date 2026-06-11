//! Patch approval lifecycle mapping.

use serde_json::json;

use crate::session::thread_store::ThreadEvent;

use super::mapper::ThreadEventMapper;

impl ThreadEventMapper {
    pub(super) fn patch_started(&mut self, patch_id: &str, arguments: &str) -> ThreadEvent {
        self.event(
            "patch.started",
            json!({
                "patch_id": patch_id,
                "arguments": arguments,
            }),
        )
    }

    pub(super) fn patch_completed(
        &mut self,
        patch_id: &str,
        metadata: &serde_json::Value,
    ) -> ThreadEvent {
        self.event(
            "patch.completed",
            json!({
                "patch_id": patch_id,
                "files": metadata.get("patch_files"),
                "hunks": metadata.get("patch_hunks"),
                "preview": metadata.get("patch_preview"),
                "approval_required": metadata.get("approval_required"),
            }),
        )
    }

    /// Map a patch approval request.
    pub fn patch_approval_required(&mut self, approval_id: &str, patch_id: &str) -> ThreadEvent {
        self.event(
            "patch.approval_required",
            json!({
                "approval_id": approval_id,
                "patch_id": patch_id,
            }),
        )
    }
}
