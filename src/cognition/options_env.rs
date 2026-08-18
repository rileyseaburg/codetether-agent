//! Runtime options assembled from environment variables.

use super::env::{env_bool, env_u32, env_u64, env_usize};
use super::{CognitionRuntimeOptions, PersonaPolicy};

/// Read runtime options from `CODETETHER_COGNITION_*` variables.
pub(super) fn options_from_env() -> CognitionRuntimeOptions {
    CognitionRuntimeOptions {
        enabled: env_bool("CODETETHER_COGNITION_ENABLED", true),
        loop_interval_ms: env_u64("CODETETHER_COGNITION_LOOP_INTERVAL_MS", 2_000),
        max_events: env_usize("CODETETHER_COGNITION_MAX_EVENTS", 2_000),
        max_snapshots: env_usize("CODETETHER_COGNITION_MAX_SNAPSHOTS", 128),
        default_policy: policy_from_env(),
    }
}

/// Read the default persona policy from the environment.
fn policy_from_env() -> PersonaPolicy {
    PersonaPolicy {
        max_spawn_depth: env_u32("CODETETHER_COGNITION_MAX_SPAWN_DEPTH", 4),
        max_branching_factor: env_u32("CODETETHER_COGNITION_MAX_BRANCHING_FACTOR", 4),
        token_budget_per_minute: env_u32("CODETETHER_COGNITION_TOKEN_BUDGET_PER_MINUTE", 20_000),
        compute_ms_per_minute: env_u32("CODETETHER_COGNITION_COMPUTE_MS_PER_MINUTE", 10_000),
        idle_ttl_secs: env_u64("CODETETHER_COGNITION_IDLE_TTL_SECS", 3_600),
        share_memory: env_bool("CODETETHER_COGNITION_SHARE_MEMORY", false),
        allowed_tools: Vec::new(),
    }
}
