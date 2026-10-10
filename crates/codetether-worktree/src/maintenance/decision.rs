use super::{WorktreeCleanupState, checks, record::WorktreeRecord};
use std::path::Path;

pub(super) async fn state(
    repo: &Path,
    record: &WorktreeRecord,
    base: &str,
    current: &Path,
) -> Result<WorktreeCleanupState, String> {
    if record.primary {
        return Ok(WorktreeCleanupState::Primary);
    }
    if checks::same_path(&record.path, current) {
        return Ok(WorktreeCleanupState::Current);
    }
    if record.locked {
        return Ok(WorktreeCleanupState::Locked);
    }
    if record.prunable {
        return if checks::merged(repo, &record.head, base).await? {
            Ok(WorktreeCleanupState::Prunable)
        } else {
            Ok(WorktreeCleanupState::Unmerged)
        };
    }
    if checks::dirty(&record.path).await? {
        return Ok(WorktreeCleanupState::Dirty);
    }
    if checks::merged(repo, &record.head, base).await? {
        Ok(WorktreeCleanupState::Ready)
    } else {
        Ok(WorktreeCleanupState::Unmerged)
    }
}
