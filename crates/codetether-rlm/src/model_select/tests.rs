use crate::config::RlmConfig;

use super::{RlmModelPurpose, RlmModelSource, select_rlm_model_with_env};

#[test]
fn root_config_wins_over_env() {
    let cfg = RlmConfig {
        root_model: Some("openai/gpt-5".into()),
        ..Default::default()
    };
    let choice = select_rlm_model_with_env(
        &cfg,
        RlmModelPurpose::Compression,
        "foreground/m",
        Some("env/m"),
        |_| None,
    );
    assert_eq!(choice.model, "openai/gpt-5");
    assert_eq!(choice.source, RlmModelSource::RootConfig);
}

#[test]
fn subcall_config_wins_for_subcalls() {
    let cfg = RlmConfig {
        root_model: Some("root/m".into()),
        subcall_model: Some("sub/m".into()),
        ..Default::default()
    };
    let choice =
        select_rlm_model_with_env(&cfg, RlmModelPurpose::Subcall, "foreground/m", None, |_| None);
    assert_eq!(choice.model, "sub/m");
    assert_eq!(choice.source, RlmModelSource::SubcallConfig);
}

#[test]
fn foreground_model_is_default() {
    let cfg = RlmConfig::default();
    let choice =
        select_rlm_model_with_env(&cfg, RlmModelPurpose::Background, "foreground/m", None, |_| {
            Some(2.0)
        });
    assert_eq!(choice.model, "foreground/m");
    assert_eq!(choice.source, RlmModelSource::Caller);
}
