//! Application forage regression coverage.
use super::*;

#[test]
fn swarm_config_preserves_explicit_model_override() {
    let args = args::new(
        "swarm".to_string(),
        Some("openai-codex/gpt-5-mini".to_string()),
    );

    let config = build_swarm_config(&args);
    assert_eq!(config.model.as_deref(), Some("openai-codex/gpt-5-mini"));
}
