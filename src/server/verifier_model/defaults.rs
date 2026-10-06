//! Read the same environment and on-disk defaults as the verifier model resolver.

#[derive(Clone, Default)]
pub(super) struct Defaults {
    pub environment: Option<String>,
    pub configured: Option<String>,
}

impl Defaults {
    pub(super) async fn load() -> anyhow::Result<Self> {
        Ok(Self {
            environment: std::env::var(crate::tool::goal::verify::VERIFIER_MODEL_ENV).ok(),
            configured: crate::config::Config::load().await?.default_model,
        })
    }
}
