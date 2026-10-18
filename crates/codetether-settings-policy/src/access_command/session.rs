//! Apply workspace configuration after the process override changes.
use super::Host;
use codetether_config::Config;
use std::path::Path;
pub(super) async fn apply(cwd: &Path, host: &mut impl Host) {
    if let Ok(cfg) = Config::load_for_workspace(cwd).await {
        codetether_trust_status::set_from_config(&cfg);
        host.apply_session_config(&cfg);
    }
}
