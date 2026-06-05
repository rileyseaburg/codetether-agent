use super::Config;

#[test]
fn migrates_legacy_kimi_default_to_minimax_m3() {
    let mut cfg = Config {
        default_provider: Some("moonshotai".to_string()),
        default_model: Some("moonshotai/kimi-k2.5".to_string()),
        ..Default::default()
    };
    cfg.normalize_legacy_defaults();
    assert_eq!(cfg.default_provider.as_deref(), Some("minimax"));
    assert_eq!(cfg.default_model.as_deref(), Some("minimax/MiniMax-M3"));
}

#[test]
fn preserves_explicit_non_legacy_default_model() {
    let mut cfg = Config {
        default_provider: Some("openai".to_string()),
        default_model: Some("openai/gpt-4o".to_string()),
        ..Default::default()
    };
    cfg.normalize_legacy_defaults();
    assert_eq!(cfg.default_provider.as_deref(), Some("openai"));
    assert_eq!(cfg.default_model.as_deref(), Some("openai/gpt-4o"));
}

#[test]
fn normalizes_zhipuai_aliases_to_current_default() {
    let mut cfg = Config {
        default_provider: Some("zhipuai".to_string()),
        default_model: Some("zhipuai/glm-5".to_string()),
        ..Default::default()
    };
    cfg.normalize_legacy_defaults();
    assert_eq!(cfg.default_provider.as_deref(), Some("minimax"));
    assert_eq!(cfg.default_model.as_deref(), Some("minimax/MiniMax-M3"));
}
