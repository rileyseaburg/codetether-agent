//! Swarm config construction from task metadata.

use crate::swarm::SwarmConfig;

use super::{
    metadata_bool, metadata_u64, metadata_usize, parse_swarm_strategy, resolve_swarm_model,
};

pub(super) struct SwarmSetup {
    pub(super) config: SwarmConfig,
    pub(super) strategy: crate::swarm::DecompositionStrategy,
}

pub(super) async fn build_swarm_setup(
    session: &mut crate::session::Session,
    metadata: &serde_json::Map<String, serde_json::Value>,
    explicit_model: Option<String>,
    model_tier: Option<&str>,
) -> SwarmSetup {
    let strategy = parse_swarm_strategy(metadata);
    let config = SwarmConfig {
        max_subagents: metadata_usize(
            metadata,
            &["swarm_max_subagents", "max_subagents", "subagents"],
        )
        .unwrap_or(10)
        .clamp(1, 100),
        max_steps_per_subagent: metadata_usize(
            metadata,
            &[
                "swarm_max_steps_per_subagent",
                "max_steps_per_subagent",
                "max_steps",
            ],
        )
        .unwrap_or(50)
        .clamp(1, 200),
        subagent_timeout_secs: metadata_u64(
            metadata,
            &["swarm_timeout_secs", "timeout_secs", "timeout"],
        )
        .unwrap_or(600)
        .clamp(30, 3600),
        parallel_enabled: metadata_bool(metadata, &["swarm_parallel_enabled", "parallel_enabled"])
            .unwrap_or(true),
        model: resolve_swarm_model(explicit_model, model_tier).await,
        working_dir: session
            .metadata
            .directory
            .as_ref()
            .map(|path| path.to_string_lossy().to_string()),
        ..Default::default()
    };
    if let Some(model) = config.model.clone() {
        session.metadata.model = Some(model);
    }
    SwarmSetup { config, strategy }
}
