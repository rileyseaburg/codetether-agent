//! Map CLI options to application swarm configuration and strategy.
use super::*;
use crate::swarm::{DecompositionStrategy, ExecutionMode, SwarmConfig};

pub(super) fn build_swarm_config(args: &ForageArgs) -> SwarmConfig {
    SwarmConfig {
        max_subagents: args.swarm_max_subagents.max(1),
        max_steps_per_subagent: args.swarm_max_steps.max(1),
        subagent_timeout_secs: args.swarm_subagent_timeout_secs.clamp(30, 86_400),
        model: args.model.clone(),
        execution_mode: ExecutionMode::LocalThread,
        ..Default::default()
    }
}

pub(super) fn parse_swarm_strategy(value: &str) -> DecompositionStrategy {
    match value {
        "domain" => DecompositionStrategy::ByDomain,
        "data" => DecompositionStrategy::ByData,
        "stage" => DecompositionStrategy::ByStage,
        "none" => DecompositionStrategy::None,
        _ => DecompositionStrategy::Automatic,
    }
}
