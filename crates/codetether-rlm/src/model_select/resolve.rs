//! Selection implementation for RLM models.

use crate::config::RlmConfig;

use super::{RLM_MODEL_ENV, RlmModelChoice, RlmModelPurpose, RlmModelSource, configured::configured};

/// Select the model for an RLM call surface.
pub fn select_rlm_model(
    config: &RlmConfig,
    purpose: RlmModelPurpose,
    caller_model: &str,
    score: impl Fn(&str) -> Option<f64>,
) -> RlmModelChoice {
    let env_model = std::env::var(RLM_MODEL_ENV).ok();
    select_rlm_model_with_env(config, purpose, caller_model, env_model.as_deref(), score)
}

/// Select an RLM model with an injected env value for deterministic tests.
pub fn select_rlm_model_with_env(
    config: &RlmConfig,
    purpose: RlmModelPurpose,
    caller_model: &str,
    env_model: Option<&str>,
    _score: impl Fn(&str) -> Option<f64>,
) -> RlmModelChoice {
    if let Some((model, source)) = configured(config, purpose) {
        return RlmModelChoice::new(model, source);
    }
    if let Some(model) = env_model.map(str::trim).filter(|s| !s.is_empty()) {
        return RlmModelChoice::new(model, RlmModelSource::Env);
    }
    RlmModelChoice::new(caller_model, RlmModelSource::Caller)
}
