use crate::config::{AccessMode, Config};
use crate::provider::ProviderRegistry;
use crate::session::Session;

pub(super) fn apply(
    session: &mut Session,
    mut config: anyhow::Result<Config>,
    registry: Option<&ProviderRegistry>,
    access_mode: Option<AccessMode>,
) {
    if let Ok(cfg) = config.as_mut() {
        Config::apply_process_access_mode_override(access_mode);
        cfg.apply_access_mode_override(access_mode);
        crate::tui::ui::trust_status::set_from_config(cfg);
        session.apply_config(cfg, registry);
    }
}

pub(super) fn apply_startup(
    session: &mut Session,
    startup: &mut super::startup::Startup,
    access_mode: Option<AccessMode>,
) {
    apply(
        session,
        std::mem::replace(&mut startup.config, Ok(Config::default())),
        startup.registry.as_deref(),
        access_mode,
    );
}
