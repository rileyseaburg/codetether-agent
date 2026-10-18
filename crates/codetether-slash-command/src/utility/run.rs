//! Access-mode precedence and utility aliases.
use super::Host;
/// Route a session utility command, returning whether it was consumed.
/// # Arguments
/// Receives the native actions and raw command text.
/// # Returns
/// True after access-mode handling or a recognized utility action.
/// # Examples
/// ```no_run
/// async fn route<H: codetether_slash_command::utility::Host>(host: &mut H) {
///     codetether_slash_command::utility::run(host, "/status").await;
/// }
/// ```
pub async fn run(host: &mut impl Host, prompt: &str) -> bool {
    if host.access_mode(prompt).await {
        return true;
    }
    match prompt.split_whitespace().next().unwrap_or("") {
        "/status" => host.status("Status").await,
        "/permissions" => host.status("Permissions").await,
        "/diff" => host.diff(),
        "/copy" => host.copy(prompt),
        "/review" => host.review(),
        "/clear" => reroute(host, "/new").await,
        "/resume" => reroute(host, "/sessions").await,
        _ => return false,
    }
    true
}
async fn reroute(host: &mut impl Host, command: &str) {
    host.dispatch(command).await;
    host.clear_input();
}
