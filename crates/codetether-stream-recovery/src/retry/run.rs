use super::Host;
use std::time::{Duration, Instant};

/// Submit a pending same-model reconnect when the UI and session are ready.
///
/// # Arguments
/// * `host` - Native session ownership, state and runtime operations.
/// * `provider` - Available provider registry, if configured.
/// # Returns
/// Completes after submission or returns early with the prompt retained when
/// the provider or session is unavailable. Busy UI state leaves it untouched.
/// # Examples
/// ```text
/// retry::execute(&mut native_host, registry.as_ref()).await;
/// ```
pub async fn execute<H: Host>(host: &mut H, provider: Option<&H::Provider>) {
    if *host.state().processing {
        return;
    }
    let Some(prompt) = host.state().recovery.pending_stream_reconnect.take() else {
        return;
    };
    let Some(provider) = provider else {
        host.state().recovery.pending_stream_reconnect = Some(prompt);
        return;
    };
    let Some(session) = host.take_session() else {
        host.state().recovery.pending_stream_reconnect = Some(prompt);
        return;
    };
    host.banner();
    *host.state().processing = true;
    let secs = crate::backoff_secs(host.state().recovery.stream_reconnect_count);
    host.wait(Duration::from_secs(secs)).await;
    host.begin_timing();
    *host.state().inflight = Some(prompt.clone());
    *host.state().last_event = Some(Instant::now());
    let request = H::request(session, prompt, provider);
    if let Err(req) = host.submit(request).await {
        host.restore(req);
        *host.state().processing = false;
    }
}
