use super::Host;
/// Route one Enter-key submission through the native handlers in priority order.
/// # Arguments
/// * `host` — The current input/session state and its native handler bindings.
/// # Returns
/// Completes after a handler consumes the input or ordinary submission finishes.
/// # Examples
/// ```ignore
/// submit(&mut native_chat).await;
/// ```
pub async fn submit<H: Host>(host: &mut H) {
    if host.goal_command().await || host.goal_answer().await || host.approval_feedback() {
        return;
    }
    let prompt = host.capture();
    if host.managed(&prompt).await {
        return;
    }
    let prompt = host.mentions(&prompt);
    if host.continuation(&prompt).await {
        return;
    }
    if host.slash(&prompt).await {
        return;
    }
    if host.processing() {
        host.steer(&prompt);
        return;
    }
    host.finish(&prompt).await;
}
