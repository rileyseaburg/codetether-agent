use crate::config::Config;
use crate::provider::ProviderRegistry;
use crate::session::Session;

pub(super) fn apply(
    session: &mut Session,
    config: anyhow::Result<Config>,
    registry: Option<&ProviderRegistry>,
) {
    if let Ok(cfg) = config {
        session.apply_config(&cfg, registry);
    }
}
