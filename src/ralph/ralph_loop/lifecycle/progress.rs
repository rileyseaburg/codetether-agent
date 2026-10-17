//! Supply application paths to shared progress-file operations.
use super::*;
impl RalphLoop {
    /// Load progress file
    pub(in crate::ralph::ralph_loop) fn load_progress(&self) -> anyhow::Result<String> {
        codetether_ralph_core::progress::load_progress(
            &self.state.working_dir,
            &self.config.progress_path,
        )
    }

    /// Append to progress file
    pub(in crate::ralph::ralph_loop) fn append_progress(
        &self,
        entry: &ProgressEntry,
        response: &str,
    ) -> anyhow::Result<()> {
        codetether_ralph_core::progress::append_progress(
            &self.state.working_dir,
            &self.config.progress_path,
            entry,
            response,
        )
    }
}
