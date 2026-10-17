//! Serde defaults for Ralph runtime configuration.

pub(crate) fn default_prd_path() -> String {
    "prd.json".to_string()
}

pub(crate) fn default_max_iterations() -> usize {
    10
}

pub(crate) fn default_progress_path() -> String {
    "progress.txt".to_string()
}

pub(crate) fn default_auto_commit() -> bool {
    false
}

pub(crate) fn default_quality_checks_enabled() -> bool {
    true
}

pub(crate) fn default_parallel_enabled() -> bool {
    true
}

pub(crate) fn default_max_concurrent_stories() -> usize {
    100
}

pub(crate) fn default_worktree_enabled() -> bool {
    true
}

pub(crate) fn default_story_timeout_secs() -> u64 {
    300
}

pub(crate) fn default_max_steps_per_story() -> usize {
    50
}

pub(crate) fn default_conflict_timeout_secs() -> u64 {
    120
}

pub(crate) fn default_relay_max_agents() -> usize {
    8
}

pub(crate) fn default_relay_max_rounds() -> usize {
    3
}

pub(crate) fn default_max_quality_retries() -> usize {
    2
}

pub(crate) fn default_max_rate_limit_retries() -> usize {
    3
}

pub(crate) fn default_rate_limit_base_delay_ms() -> u64 {
    2000
}
