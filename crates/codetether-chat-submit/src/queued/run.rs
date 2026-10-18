use super::Host;
/// Deliver goal input and submit one queued prompt when all gates allow it.
/// # Arguments
/// Host supplies goal handlers, execution state, queue and input dispatch.
/// # Returns
/// Completes after one prompt is submitted or a gate prevents submission.
/// # Examples
/// ```ignore
/// drain(&mut native_ui).await;
/// ```
pub async fn drain<H: Host>(host: &mut H) {
    host.commands().await;
    host.answers().await;
    if host.processing() || !host.has_pending() || host.held() {
        return;
    }
    let Some(prompt) = host.take() else {
        return;
    };
    let state = host.input();
    *state.input = prompt;
    *state.cursor = state.input.chars().count();
    *state.status = "Running queued prompt".to_string();
    host.enter().await;
}
