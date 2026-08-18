//! Tests for tool-router configuration defaults.

use super::ToolRouterConfig;

#[test]
fn config_defaults_disabled() {
    let config = ToolRouterConfig::default();
    assert!(!config.enabled);
    assert_eq!(config.arch, "gemma3");
    assert_eq!(config.max_tokens, 128);
}
