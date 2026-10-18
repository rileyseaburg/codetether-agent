//! Boolean setting command dispatch and status display.
use super::{Host, Setting, parse};
/// Execute one boolean preference command against the native client.
/// # Arguments
/// * `host` - Preference state, setter and status bindings.
/// * `setting` - Preference selected by the slash command.
/// * `rest` - Optional on/off/toggle/status action.
/// # Returns
/// Completes after applying the preference or displaying status/usage.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_settings_policy::command::Host>(host: &mut H) {
/// codetether_settings_policy::command::execute(host, codetether_settings_policy::command::Setting::Network, "status").await;
/// # }
/// ```
pub async fn execute<H: Host>(host: &mut H, setting: Setting, rest: &str) {
    let current = host.current(setting);
    let desired = match parse::desired(rest, current) {
        Ok(desired) => desired,
        Err(()) => {
            host.set_status(format!(
                "Usage: /{} [on|off|toggle|status]",
                setting.command()
            ));
            return;
        }
    };
    if let Some(next) = desired {
        host.apply(setting, next).await;
    } else {
        host.set_status(setting.status(current));
    }
}
