use super::Host;
/// Gate and prepare a prompt before configured-provider handoff.
/// # Arguments
/// * `host` — Native effects and disjoint dispatch state.
/// * `registry` — Optional borrowed provider handle.
/// * `prompt` — Expanded prompt text.
/// * `pending_images` — Images moved into provider execution.
/// # Returns
/// Completes after rejection, handoff or missing-provider handling.
/// # Examples
/// ```ignore
/// dispatch_prompt(&mut host, registry.as_ref(), &prompt, images).await;
/// ```
pub async fn dispatch_prompt<H: Host>(
    host: &mut H,
    registry: Option<&H::Provider>,
    prompt: &str,
    pending_images: Vec<H::Image>,
) {
    if host.held() {
        *host.state().status =
            "Goal paused — select Yes in the answer review or submit /continue".into();
        return;
    }
    host.clear_input();
    host.processing_started().await;
    host.begin_timing();
    // A genuine new user prompt restores the full watchdog auto-restart budget.
    // (The count intentionally survives completed turns; see session_events done.)
    *host.state().watchdog_restarts = 0;
    *host.state().inflight_prompt = Some(prompt.to_string());
    *host.state().status = "Submitting prompt…".to_string();
    host.scroll();

    if let Some(reg) = registry {
        host.spawn(reg, prompt, pending_images).await;
    } else {
        host.no_provider().await;
    }
}
