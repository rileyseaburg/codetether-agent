use super::{Host, idle, prompt, stalled};
/// Handle exactly /continue and choose stalled recovery or idle dispatch.
/// # Arguments
/// Host supplies native state; prompt is the raw submitted command.
/// # Returns
/// True when the command is recognized, false for ordinary routing fallback.
/// # Examples
/// ```ignore
/// let handled = run(&mut host, "/continue").await;
/// ```
pub async fn run<H: Host>(host: &mut H, command: &str) -> bool {
    if command.trim() != "/continue" {
        return false;
    }
    let prompt = prompt::resolve(host);
    host.clear_input();
    if host.is_processing() || host.session().is_none() {
        stalled::resume(host, prompt).await;
    } else {
        idle::resume(host, prompt).await;
    }
    true
}
