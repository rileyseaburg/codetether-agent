//! Existing autonomous run defaults and caller iteration limit.

use codetether_ralph_core::RalphConfig;
use std::path::Path;
pub(crate) fn build(prd_path: &Path, model: &str, max_iterations: Option<usize>) -> RalphConfig {
    RalphConfig {
        prd_path: prd_path.to_string_lossy().to_string(),
        max_iterations: max_iterations.unwrap_or(10),
        progress_path: "progress.txt".to_string(),
        quality_checks_enabled: true,
        auto_commit: true,
        model: Some(model.to_owned()),
        use_rlm: false,
        parallel_enabled: true,
        max_concurrent_stories: 3,
        worktree_enabled: true,
        story_timeout_secs: 300,
        conflict_timeout_secs: 120,
        relay_enabled: false,
        relay_max_agents: 8,
        relay_max_rounds: 3,
        max_steps_per_story: 30,
        ..Default::default()
    }
}
