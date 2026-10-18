//! Map CLI options to application swarm configuration and strategy.
use crate::*;
use codetether_swarm_core::{DecompositionStrategy, ExecutionMode, SwarmConfig};

/// Build local swarm settings with the existing forage limits.
/// # Arguments
/// * `args` — Model override and swarm resource limits.
/// # Returns
/// Local-thread configuration with positive worker/step limits and bounded timeout.
/// # Examples
/// ```no_run
/// # fn configure(args: &codetether_cli_options::ForageArgs) {
/// let config = codetether_forage_execution::swarm_config::build_swarm_config(args);
/// assert!(config.max_subagents >= 1);
/// # }
/// ```
pub fn build_swarm_config(args: &ForageArgs) -> SwarmConfig {
    SwarmConfig {
        max_subagents: args.swarm_max_subagents.max(1),
        max_steps_per_subagent: args.swarm_max_steps.max(1),
        subagent_timeout_secs: args.swarm_subagent_timeout_secs.clamp(30, 86_400),
        model: args.model.clone(),
        execution_mode: ExecutionMode::LocalThread,
        ..Default::default()
    }
}

/// Select the existing swarm decomposition strategy from its CLI value.
/// # Arguments
/// * `value` — Strategy name supplied by forage options.
/// # Returns
/// The requested strategy, defaulting to automatic for unknown values.
/// # Examples
/// ```
/// use codetether_forage_execution::swarm_config::parse_swarm_strategy;
/// use codetether_swarm_core::DecompositionStrategy;
/// assert!(matches!(parse_swarm_strategy("none"), DecompositionStrategy::None));
/// ```
pub fn parse_swarm_strategy(value: &str) -> DecompositionStrategy {
    match value {
        "domain" => DecompositionStrategy::ByDomain,
        "data" => DecompositionStrategy::ByData,
        "stage" => DecompositionStrategy::ByStage,
        "none" => DecompositionStrategy::None,
        _ => DecompositionStrategy::Automatic,
    }
}
