//! Build the existing autonomous Ralph configuration.

use codetether_okr::OkrRun;
use codetether_ralph_core::RalphConfig;
use std::path::Path;
pub(crate) fn build(
    prd_path: &Path,
    okr_run: &OkrRun,
    model: &str,
    max_iterations: usize,
    max_concurrent_stories: usize,
) -> RalphConfig {
    RalphConfig {
        prd_path: prd_path.to_string_lossy().to_string(),
        max_iterations,
        progress_path: format!("progress_{}.txt", okr_run.id.to_string().replace('-', "_")),
        quality_checks_enabled: true,
        auto_commit: true,
        model: Some(model.to_string()),
        use_rlm: false,
        parallel_enabled: true,
        max_concurrent_stories,
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
