use super::{WorktreeCleanupEntry, WorktreeCleanupState, decision, record::WorktreeRecord};
use crate::worktree::WorktreeManager;
use std::path::Path;

impl WorktreeManager {
    pub(super) async fn inspect_registered(
        &self,
        record: WorktreeRecord,
        base: &str,
        current: &Path,
    ) -> WorktreeCleanupEntry {
        let state = decision::state(&self.repo_path, &record, base, current).await;
        let (state, error) = match state {
            Ok(state) => (state, None),
            Err(error) => (WorktreeCleanupState::InspectionFailed, Some(error)),
        };
        WorktreeCleanupEntry {
            path: record.path,
            branch: record.branch,
            state,
            error,
        }
    }
}
