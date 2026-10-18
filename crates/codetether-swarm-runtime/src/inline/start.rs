//! Worker-facing routing and configuration feedback.
use std::sync::Arc;

/// Publish routing and configuration feedback when output is enabled.
/// # Arguments
/// Receives the sink, strategy/configuration and resolved worker routing labels.
/// # Returns
/// Emits two messages or does nothing when no sink is present.
/// # Examples
/// ```
/// codetether_swarm_runtime::inline::emit_start(&None, codetether_swarm_core::DecompositionStrategy::None, &Default::default(), None, None, None, None);
/// ```
pub fn emit_start(
    cb: &Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
    strategy: codetether_swarm_core::DecompositionStrategy,
    config: &codetether_swarm_core::SwarmConfig,
    tier: Option<&str>,
    complexity: Option<&str>,
    personality: Option<&str>,
    target: Option<&str>,
) {
    if let Some(cb) = cb {
        cb(format!(
            "[swarm] routing complexity={} tier={} personality={} target_agent={}",
            complexity.unwrap_or("standard"),
            tier.unwrap_or("balanced"),
            personality.unwrap_or("auto"),
            target.unwrap_or("auto")
        ));
        cb(format!(
            "[swarm] config strategy={:?} max_subagents={} max_steps={} timeout={}s tier={}",
            strategy,
            config.max_subagents,
            config.max_steps_per_subagent,
            config.subagent_timeout_secs,
            tier.unwrap_or("balanced")
        ));
    }
}
