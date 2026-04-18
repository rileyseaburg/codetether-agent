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
    let mode = mode::from_request(request)?;
    let mut slot = session.inner.runtime.lock().await;
    if slot.as_ref().is_some_and(SessionRuntime::is_alive) {
        return Ok(BrowserOutput::Ack(Ack { ok: true }));
    }
    if let Some(runtime) = slot.take() {
        stop::shutdown(runtime).await?;
    }
    *slot = Some(launch::runtime(mode).await?);
    Ok(BrowserOutput::Ack(Ack { ok: true }))
}
