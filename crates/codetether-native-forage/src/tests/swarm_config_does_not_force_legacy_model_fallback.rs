//! Application forage regression coverage.
use super::*;

#[test]
fn swarm_config_does_not_force_legacy_model_fallback() {
    let args = args::new("swarm".to_string(), None);

    let config = build_swarm_config(&args);
    assert!(config.model.is_none());
}
