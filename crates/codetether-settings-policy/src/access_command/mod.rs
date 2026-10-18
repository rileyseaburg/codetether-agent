//! Access-mode command parsing, application and presentation.
//!
//! [run] preserves verifier-command precedence and access-mode aliases, then
//! updates the process override, reloads config and presents release status.
use codetether_config::Config;
use std::path::Path;
mod binding;
mod host;
mod parse;
mod policy;
mod result;
mod session;
pub use host::Host;
/// Handle access-mode aliases after giving verifier commands precedence.
/// # Arguments
/// * `host` — Existing session, approval and frontend operations.
/// * `cwd` — Workspace used for configuration reload.
/// * `prompt` — Submitted command text.
/// # Returns
/// Whether the command was consumed, including invalid mode arguments.
/// # Examples
/// ```no_run
/// # async fn example(host: &mut impl codetether_settings_policy::access_command::Host) {
/// let _ = codetether_settings_policy::access_command::run(host, std::path::Path::new("."), "/access-mode ask").await;
/// # }
/// ```
pub async fn run(host: &mut impl Host, cwd: &Path, prompt: &str) -> bool {
    if host.verifier_command(prompt) {
        return true;
    }
    let Some((command, rest)) = parse::prompt(prompt) else {
        return false;
    };
    let rest = rest.trim();
    if rest.is_empty() && command == "/permissions" {
        return false;
    }
    let Some(mode) = parse::mode(rest) else {
        host.set_status("Usage: /access-mode <ask|approve|full>".to_string());
        return true;
    };
    Config::apply_process_access_mode_override(Some(mode));
    session::apply(cwd, host).await;
    result::push(host, cwd, mode).await;
    true
}
