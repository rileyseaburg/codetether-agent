use crate::worktree::WorktreeManager;

/// Markers for git repair failures that are expected and non-actionable.
const BENIGN_MARKERS: &[&str] = &[
    "gc is already running",
    "there are too many unreachable loose objects",
];

impl WorktreeManager {
    /// Return true when a failed repair step is benign and should not warn.
    pub(crate) fn is_benign_repair_failure(output: &str) -> bool {
        let lower = output.to_ascii_lowercase();
        BENIGN_MARKERS.iter().any(|marker| lower.contains(marker))
    }

    /// Log a non-zero-exit repair step, downgrading benign failures to debug.
    pub(crate) fn log_repair_failure(&self, command: &str, details: &str) {
        let summary = Self::summarize_git_output(details);
        let repo_path = self.repo_path.display().to_string();
        if Self::is_benign_repair_failure(details) {
            tracing::debug!(
                repo_path = %repo_path,
                command = %command,
                error = %summary,
                "Git repair step skipped (benign failure)"
            );
            return;
        }
        tracing::warn!(
            repo_path = %repo_path,
            command = %command,
            error = %summary,
            "Git repair step failed"
        );
    }
}
