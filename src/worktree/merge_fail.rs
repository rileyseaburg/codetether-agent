use super::{MergeResult, WorktreeManager};
use anyhow::{Result, anyhow};
use std::process::Output;

impl WorktreeManager {
    pub(crate) async fn failed_merge_result(
        &self,
        output: Output,
        stashed: bool,
    ) -> Result<MergeResult> {
        let has_conflict = Self::merge_output_has_conflict(&output);
        self.abort_merge_state().await;
        self.pop_stash_if(stashed);
        if !has_conflict {
            return Err(anyhow!(
                "Git merge failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        Ok(MergeResult {
            success: false,
            aborted: false,
            conflicts: self.get_conflict_list().await?,
            conflict_diffs: self.get_conflict_diffs().await?,
            files_changed: 0,
            summary: "Merge has conflicts that need resolution".to_string(),
        })
    }
}
