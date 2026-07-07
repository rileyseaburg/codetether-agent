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
        startup
            .config
            .take()
            .unwrap_or_else(|| Ok(Config::default())),
        startup.registry.as_deref(),
        access_mode,
    );
}

/// Apply startup access-mode + config + `--yolo` policy in the correct order.
///
/// Access mode is resolved first (yolo forces `Full`), config is applied, then
/// the yolo session flag is set last so it is never clobbered by
/// [`Session::apply_config`].
pub(super) fn apply_policies(
    session: &mut Session,
    startup: &mut super::startup::Startup,
    access_mode: Option<AccessMode>,
    yolo: bool,
) {
    let access_mode = super::full_auto::effective_access_mode(access_mode, yolo);
    apply_startup(session, startup, access_mode);
    super::full_auto::apply_session_policy(session, yolo);
}
