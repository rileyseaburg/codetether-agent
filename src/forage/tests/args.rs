//! Common CLI argument fixture used by forage adapter tests.
use super::*;
pub(super) fn new(execution_engine: String, model: Option<String>) -> ForageArgs {
    ForageArgs {
        top: 3,
        loop_mode: false,
        interval_secs: 120,
        max_cycles: 1,
        execute: true,
        moonshots: Vec::new(),
        moonshot_file: None,
        moonshot_required: false,
        moonshot_min_alignment: 0.10,
        execution_engine,
        run_timeout_secs: 900,
        fail_fast: false,
        swarm_strategy: "auto".to_string(),
        swarm_max_subagents: 8,
        swarm_max_steps: 100,
        swarm_subagent_timeout_secs: 300,
        model,
        json: false,
        no_s3: false,
    }
}
