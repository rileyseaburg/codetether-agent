//! Translate a Ralph PRD into shared workspace inputs.
use super::BenchmarkRunner;
use crate::ralph::Prd;
use std::path::{Path, PathBuf};
impl BenchmarkRunner {
    pub(super) fn scaffold_workspace(
        prd_path: &Path,
        prd: &Prd,
    ) -> anyhow::Result<(tempfile::TempDir, PathBuf)> {
        codetether_benchmark::workspace::scaffold_workspace(
            prd_path,
            &prd.project,
            &prd.feature,
            prd.user_stories
                .iter()
                .map(|story| (story.title.as_str(), story.description.as_str())),
        )
    }
}
