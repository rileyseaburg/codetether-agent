//! Access override, configuration reload and approval release ordering.
use super::Host;
use codetether_config::{AccessMode, Config};
/// Advance access mode and apply it to the current interactive session.
/// # Arguments
/// * `host` - Existing trust, session and approval bindings.
/// # Returns
/// Completes after updating status, including approval release errors if any.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_settings_policy::access_update::Host>(host: &mut H) {
/// codetether_settings_policy::access_update::cycle(host).await;
/// # }
/// ```
pub async fn cycle<H: Host>(host: &mut H) {
    let next = crate::next_access_mode(host.current_mode());
    Config::apply_process_access_mode_override(Some(next));
    let cwd = std::env::current_dir().unwrap_or_default();
    if let Ok(cfg) = Config::load_for_workspace(&cwd).await {
        host.apply_config(&cfg);
    }
    let status = if next == AccessMode::Full {
        match host.release_all() {
            Ok(count) => format!("Access mode: full ({count} paused request(s) released)"),
            Err(error) => format!("Access mode: full; paused request release failed: {error}"),
        }
    } else {
        format!("Access mode: {}", crate::access_label(next))
    };
    host.set_status(status);
}
