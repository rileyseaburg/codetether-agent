use super::WorktreeManager;
use anyhow::anyhow;
use std::path::Path;

impl WorktreeManager {
    pub(crate) fn integrity_error_message(repo_path: &Path, fsck_output: &str) -> anyhow::Error {
        let summary = Self::summarize_git_output(fsck_output);
        anyhow!(
            "Git object database is corrupted in '{}': {}\n\
Automatic repair was attempted but repository integrity is still broken.\n\
Recovery steps:\n\
1. Backup local changes: git diff > /tmp/codetether-recovery.patch\n\
2. Attempt object recovery: git fetch --all --prune --tags && git fsck --full\n\
3. If corruption remains, create a fresh clone and re-apply the patch.",
            repo_path.display(),
            summary
        )
    }
}
