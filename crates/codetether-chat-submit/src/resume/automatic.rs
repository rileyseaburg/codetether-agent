//! Automatic continuation using the existing native continuation operations.
use super::Host;
/// Resume a durable goal prompt when the session and provider registry are idle.
/// # Arguments
/// Receives continuation operations and provider-registry availability.
/// # Returns
/// True after publishing and dispatching a durable prompt; false when ineligible.
/// # Examples
/// ```no_run
/// async fn resume<H: codetether_chat_submit::resume::Host>(host: &mut H) {
///     let _ = codetether_chat_submit::resume::automatic::run(host, true).await;
/// }
/// ```
pub async fn run<H: Host>(host: &mut H, providers_available: bool) -> bool {
    if host.is_processing() || !providers_available {
        return false;
    }
    let Some(prompt) = host.session().and_then(H::durable_prompt) else {
        return false;
    };
    host.status("Resuming active goal…".to_string());
    host.publish_user(&prompt);
    host.dispatch(&prompt).await;
    true
}
