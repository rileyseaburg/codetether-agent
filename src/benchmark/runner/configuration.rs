//! Configure Ralph for one isolated benchmark workspace.
use super::BenchmarkRunner;
use crate::ralph::RalphConfig;
use std::path::Path;
impl BenchmarkRunner {
    pub(super) fn ralph_config(&self, workspace_prd_path: &Path) -> RalphConfig {
        // Configure Ralph to run in the benchmark workspace
        let config = RalphConfig {
            prd_path: workspace_prd_path.to_string_lossy().to_string(),
            max_iterations: self.config.max_iterations,
            story_timeout_secs: self.config.story_timeout_secs,
            quality_checks_enabled: true,
            auto_commit: true, // Commit per story for clean git history
            parallel_enabled: self.config.parallel,
            ..Default::default()
        };

        config
    }
}
