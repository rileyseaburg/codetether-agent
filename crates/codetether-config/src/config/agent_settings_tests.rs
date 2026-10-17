use super::Config;

#[test]
fn parses_codex_agent_settings_and_named_profiles_together() {
    let input = r#"
        [agents]
        interrupt_message = false

        [agents.reviewer]
        name = "reviewer"
        description = "Find correctness risks"
    "#;
    let config: Config = toml::from_str(input).unwrap();
    assert!(!config.agents.interrupt_message_enabled());
    assert_eq!(config.agents.get("reviewer").unwrap().name, "reviewer");
}

#[test]
fn interruption_marker_defaults_true_and_merges_explicitly() {
    assert!(Config::default().agents.interrupt_message_enabled());
    let overlay: Config = toml::from_str("[agents]\ninterrupt_message = false").unwrap();
    let merged = Config::default().merge(overlay);
    assert!(!merged.agents.interrupt_message_enabled());
}

#[test]
fn config_set_supports_interrupt_message() {
    let mut config = Config::default();
    config
        .set_value("agents.interrupt_message", "false")
        .unwrap();
    assert!(!config.agents.interrupt_message_enabled());
}

#[test]
fn untrusted_project_cannot_change_agent_settings() {
    let overlay: Config = toml::from_str("[agents]\ninterrupt_message = false").unwrap();
    let sanitized =
        super::super::project_policy::sanitize_project_overlay(&Config::default(), overlay);
    assert!(sanitized.agents.interrupt_message_enabled());
}
