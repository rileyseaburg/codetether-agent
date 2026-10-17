//! Parse worker metadata into bounded swarm settings.
use super::*;
/// Build swarm settings with caller-supplied asynchronous model resolution.
///
/// # Arguments
/// * `metadata` - Task metadata and legacy field aliases.
/// * `directory` - Optional session workspace directory.
/// * `resolve_model` - Model selection future, awaited after resource limits are parsed.
/// # Returns
/// Swarm configuration and decomposition strategy using existing worker defaults.
/// # Examples
/// ```rust,no_run
/// # async fn example() {
/// use codetether_a2a_worker_core::swarm_setup::build_swarm_setup;
/// let setup = build_swarm_setup(&serde_json::Map::new(), None, async { None }).await;
/// assert_eq!(setup.config.max_subagents, 10);
/// # }
/// ```

pub async fn build_swarm_setup(
    metadata: &serde_json::Map<String, serde_json::Value>,
    directory: Option<&std::path::Path>,
    resolve_model: impl std::future::Future<Output = Option<String>>,
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
        model: resolve_model.await,
        working_dir: directory.map(|path| path.to_string_lossy().to_string()),
        ..Default::default()
    };
    SwarmSetup { config, strategy }
}
