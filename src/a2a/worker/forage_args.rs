//! Adapt shared forage task options to application CLI arguments.
use crate::cli::ForageArgs;
pub(super) fn build_forage_args(
    prompt: &str,
    title: &str,
    metadata: &serde_json::Map<String, serde_json::Value>,
    selected_model: Option<String>,
) -> ForageArgs {
    let options = codetether_a2a_worker_core::forage::build_forage_options(
        prompt,
        title,
        metadata,
        selected_model,
    );
    ForageArgs {
        top: options.top,
        loop_mode: options.loop_mode,
        interval_secs: options.interval_secs,
        max_cycles: options.max_cycles,
        execute: options.execute,
        no_s3: options.no_s3,
        moonshots: options.moonshots,
        moonshot_file: options.moonshot_file,
        moonshot_required: options.moonshot_required,
        moonshot_min_alignment: options.moonshot_min_alignment,
        execution_engine: options.execution_engine,
        run_timeout_secs: options.run_timeout_secs,
        fail_fast: options.fail_fast,
        swarm_strategy: options.swarm_strategy,
        swarm_max_subagents: options.swarm_max_subagents,
        swarm_max_steps: options.swarm_max_steps,
        swarm_subagent_timeout_secs: options.swarm_subagent_timeout_secs,
        model: options.model,
        json: options.json,
    }
}
