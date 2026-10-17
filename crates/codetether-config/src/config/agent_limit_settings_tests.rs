use super::Config;

#[test]
fn max_threads_alias_matches_canonical_setting() {
    let canonical: Config =
        toml::from_str("[agents]\nmax_concurrent_threads_per_session = 7").unwrap();
    let alias: Config = toml::from_str("[agents]\nmax_threads = 7").unwrap();
    assert_eq!(canonical.agents.max_threads(), 7);
    assert_eq!(alias.agents.max_threads(), 7);
    let encoded = toml::to_string(&alias).unwrap();
    assert!(encoded.contains("max_concurrent_threads_per_session = 7"));
    assert!(!encoded.contains("max_threads = 7"));
}

#[test]
fn max_threads_rejects_zero_but_depth_allows_zero() {
    assert!(toml::from_str::<Config>("[agents]\nmax_threads = 0").is_err());
    let config: Config = toml::from_str("[agents]\nmax_depth = 0").unwrap();
    assert_eq!(config.agents.max_depth(), 0);
}

#[test]
fn agent_limits_merge_and_default_like_codex() {
    let overlay: Config = toml::from_str("[agents]\nmax_threads = 3\nmax_depth = 2").unwrap();
    let merged = Config::default().merge(overlay);
    assert_eq!(merged.agents.max_threads(), 3);
    assert_eq!(merged.agents.max_depth(), 2);
    assert_eq!(Config::default().agents.max_threads(), 6);
    assert_eq!(Config::default().agents.max_depth(), 1);
}

#[test]
fn config_set_supports_both_agent_limits() {
    let mut config = Config::default();
    config.set_value("agents.max_threads", "4").unwrap();
    config.set_value("agents.max_depth", "3").unwrap();
    assert_eq!(config.agents.max_threads(), 4);
    assert_eq!(config.agents.max_depth(), 3);
    assert!(config.set_value("agents.max_threads", "0").is_err());
}
