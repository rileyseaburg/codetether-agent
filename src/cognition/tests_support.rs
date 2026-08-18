//! Shared fixtures for cognition runtime tests.

use super::{
    CognitionRuntime, CognitionRuntimeOptions, PersonaPolicy, ThoughtPhase, ThoughtWorkItem,
};

/// A work item with a fixed persona, varying only by phase.
pub(super) fn sample_work_item(phase: ThoughtPhase) -> ThoughtWorkItem {
    ThoughtWorkItem {
        persona_id: "p-1".to_string(),
        persona_name: "Spotlessbinco Business Thinker".to_string(),
        role: "principal reliability engineer".to_string(),
        charter:
            "Continuously think about /home/riley/spotlessbinco as a production business system."
                .to_string(),
        swarm_id: Some("spotlessbinco".to_string()),
        thought_count: 4,
        phase,
    }
}

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

/// A runtime using `default_policy`, with a 10 ms tick interval.
pub(super) fn runtime_with_policy(default_policy: PersonaPolicy) -> CognitionRuntime {
    CognitionRuntime::new_with_options(CognitionRuntimeOptions {
        enabled: true,
        loop_interval_ms: 10,
        max_events: 256,
        max_snapshots: 32,
        default_policy,
    })
}
