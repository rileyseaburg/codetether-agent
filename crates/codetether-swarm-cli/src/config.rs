//! Swarm execution configuration from CLI flags.
use codetether_cli_options::SwarmArgs;
use codetether_swarm_core::{ExecutionMode, SwarmConfig};
pub(super) fn build(args: &SwarmArgs) -> SwarmConfig {
    SwarmConfig {
        max_subagents: args.max_subagents,
        max_steps_per_subagent: args.max_steps,
        subagent_timeout_secs: args.timeout,
        model: args.model.clone(),
        execution_mode: ExecutionMode::from_cli_value(&args.execution_mode),
        k8s_pod_budget: args.k8s_pod_budget,
        k8s_subagent_image: args.k8s_image.clone(),
        ..Default::default()
    }
}
