//! Runtime settings retained from the native persona regression fixture.
use codetether_cognition_core::{CognitionRuntimeOptions, PersonaPolicy};
type CognitionRuntime = crate::CognitionRuntime<super::host::TestHost>;

/// A runtime with small budgets and fast ticks, suitable for tests.
pub(super) fn test_runtime() -> CognitionRuntime {
    CognitionRuntime::new_with_options(CognitionRuntimeOptions {
        enabled: true,
        loop_interval_ms: 25,
        max_events: 256,
        max_snapshots: 32,
        default_policy: PersonaPolicy {
            max_spawn_depth: 2,
            max_branching_factor: 2,
            token_budget_per_minute: 1_000,
            compute_ms_per_minute: 1_000,
            idle_ttl_secs: 300,
            share_memory: false,
            allowed_tools: Vec::new(),
        },
    })
}
