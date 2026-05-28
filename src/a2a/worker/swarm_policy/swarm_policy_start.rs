use std::sync::Arc;

pub(super) fn emit(
    cb: &Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
    strategy: crate::swarm::DecompositionStrategy,
    config: &crate::swarm::SwarmConfig,
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
