//! Apply startup config with access overrides before full-auto flags.
use super::Host;
use codetether_config::{AccessMode, Config};
/// Apply the corresponding stage of startup configuration policy.
/// # Arguments
/// Receives native policy bindings, loaded configuration or startup results,
/// and the explicitly requested access/full-auto settings.
/// # Returns
/// Applies the stage in place while preserving configuration error behavior.
pub fn apply<H: Host>(
    host: &mut H,
    mut config: anyhow::Result<Config>,
    registry: Option<&H::Registry>,
    access_mode: Option<AccessMode>,
) {
    if let Ok(cfg) = config.as_mut() {
        Config::apply_process_access_mode_override(access_mode);
        cfg.apply_access_mode_override(access_mode);
        host.set_trust(cfg);
        if let Some(registry) = registry {
            host.install_review(cfg, registry);
        }
        host.apply_config(cfg, registry);
    }
}

/// Apply the corresponding stage of startup configuration policy.
/// # Arguments
/// Receives native policy bindings, loaded configuration or startup results,
/// and the explicitly requested access/full-auto settings.
/// # Returns
/// Applies the stage in place while preserving configuration error behavior.
pub fn apply_startup<H: Host, B, T>(
    host: &mut H,
    startup: &mut crate::Startup<H::Registry, B, T>,
    access_mode: Option<AccessMode>,
) {
    apply(
        host,
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
/// [Host::apply_config].
/// Apply the corresponding stage of startup configuration policy.
/// # Arguments
/// Receives native policy bindings, loaded configuration or startup results,
/// and the explicitly requested access/full-auto settings.
/// # Returns
/// Applies the stage in place while preserving configuration error behavior.
pub fn apply_policies<H: Host, B, T>(
    host: &mut H,
    startup: &mut crate::Startup<H::Registry, B, T>,
    access_mode: Option<AccessMode>,
    yolo: bool,
) {
    let access_mode = super::full_auto::effective_access_mode(access_mode, yolo);
    apply_startup(host, startup, access_mode);
    super::full_auto::apply_session_policy(host, yolo);
}
