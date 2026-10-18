//! Read the same environment and on-disk defaults as the verifier model resolver.

#[derive(Clone, Default)]
pub(super) struct Defaults {
    pub environment: Option<String>,
    pub configured: Option<String>,
}

impl Defaults {
    pub(super) async fn load() -> anyhow::Result<Self> {
        Ok(Self {
            environment: std::env::var(codetether_verifier_state::VERIFIER_MODEL_ENV).ok(),
            configured: codetether_config::Config::load().await?.default_model,
        })
    }
}
