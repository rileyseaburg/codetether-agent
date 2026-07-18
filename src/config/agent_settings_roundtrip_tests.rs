use super::Config;

#[test]
fn agents_table_round_trips_through_toml() {
    let input = r#"
        [agents]
        interrupt_message = false
        max_threads = 4
        max_depth = 2
        [agents.worker]
        name = "worker"
    "#;
    let config: Config = toml::from_str(input).unwrap();
    let encoded = toml::to_string_pretty(&config).unwrap();
    let decoded: Config = toml::from_str(&encoded).unwrap();
    assert!(!decoded.agents.interrupt_message_enabled());
    assert_eq!(decoded.agents.max_threads(), 4);
    assert_eq!(decoded.agents.max_depth(), 2);
    assert_eq!(decoded.agents.get("worker").unwrap().name, "worker");
}
