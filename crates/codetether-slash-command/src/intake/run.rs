use super::Host;
/// Apply chat command intake and busy-session fallback in priority order.
/// # Arguments
/// Host supplies native effects and availability; prompt is raw submitted input.
/// # Returns
/// True when input was a handled shell command or any slash-prefixed command.
/// # Examples
/// ```ignore
/// let consumed = run(&mut native_ui, "/help").await;
/// ```
pub async fn run<H: Host>(host: &mut H, prompt: &str) -> bool {
    if host.shell(prompt).await {
        return true;
    }
    if !prompt.starts_with('/') {
        return false;
    }
    if host.approval(prompt) {
        return true;
    }
    if host.mux(prompt).await {
        return true;
    }
    if host.session(prompt).await {
        return true;
    }
    if host.session_free(prompt) {
        host.clear_input();
    } else {
        host.status("Session is busy; slash command was not run".to_string());
    }
    true
}
