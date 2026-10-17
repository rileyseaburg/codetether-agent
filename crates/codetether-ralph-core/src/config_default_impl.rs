//! Explicit runtime defaults, distinct from field-by-field serde defaults.
use crate::{RalphConfig, config_defaults::*};

impl Default for RalphConfig {
    fn default() -> Self {
        Self {
            prd_path: default_prd_path(),
            max_iterations: default_max_iterations(),
            progress_path: default_progress_path(),
            auto_commit: default_auto_commit(),
            quality_checks_enabled: default_quality_checks_enabled(),
            model: None,
            use_rlm: true,
            parallel_enabled: default_parallel_enabled(),
            max_concurrent_stories: default_max_concurrent_stories(),
            worktree_enabled: default_worktree_enabled(),
            story_timeout_secs: default_story_timeout_secs(),
            max_steps_per_story: default_max_steps_per_story(),
            conflict_timeout_secs: default_conflict_timeout_secs(),
            relay_enabled: true,
            relay_max_agents: default_relay_max_agents(),
            relay_max_rounds: default_relay_max_rounds(),
            max_quality_retries: default_max_quality_retries(),
            max_rate_limit_retries: default_max_rate_limit_retries(),
            rate_limit_base_delay_ms: default_rate_limit_base_delay_ms(),
        }
    }
}
