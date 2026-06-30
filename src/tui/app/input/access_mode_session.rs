use std::path::Path;
use std::sync::Arc;

use crate::config::Config;
use crate::provider::ProviderRegistry;
use crate::session::Session;

pub(super) async fn apply(
    cwd: &Path,
    session: &mut Session,
    registry: Option<&Arc<ProviderRegistry>>,
) {
    if let Ok(cfg) = Config::load_for_workspace(cwd).await {
        crate::tui::ui::trust_status::set_from_config(&cfg);
        session.apply_config(&cfg, registry.map(Arc::as_ref));
    }
}
