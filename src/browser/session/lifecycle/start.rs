use super::{launch, mode, stop};
use crate::browser::{
    BrowserError, BrowserOutput,
    output::Ack,
    request::StartRequest,
    session::{BrowserSession, SessionRuntime},
};

pub(super) async fn run(
    session: &BrowserSession,
    request: StartRequest,
) -> Result<BrowserOutput, BrowserError> {
    let mode = mode::from_request(request).await?;
    let mut slot = session.inner.runtime.lock().await;
    if slot.as_ref().is_some_and(SessionRuntime::is_alive) {
        return Ok(BrowserOutput::Ack(Ack { ok: true }));
    }
    if let Some(runtime) = slot.take() {
        stop::shutdown(runtime).await?;
    }
    // If the caller asked to launch but a browser with remote debugging is
    // already listening on the host, attach to it instead of spawning a
    // second (doomed) copy that will race the existing profile lock.
    let effective_mode = match mode {
        mode::StartMode::Launch(_) => match super::attach::detect_running_browser().await {
            Some(ws_url) => mode::StartMode::Connect { ws_url },
            None => mode,
        },
        other => other,
    };
    *slot = Some(launch::runtime(effective_mode).await?);
    Ok(BrowserOutput::Ack(Ack { ok: true }))
}
