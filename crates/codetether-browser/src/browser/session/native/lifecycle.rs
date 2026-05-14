use crate::browser::{BrowserError, BrowserOutput, output::Ack, request::StartRequest};
use serde_json::json;

pub(super) async fn start(
    session: &super::super::BrowserSession,
    _request: StartRequest,
) -> Result<BrowserOutput, BrowserError> {
    *session.inner.native.lock().await = Some(super::NativeRuntime::new());
    Ok(ack())
}

pub(super) async fn stop(
    session: &super::super::BrowserSession,
) -> Result<BrowserOutput, BrowserError> {
    *session.inner.native.lock().await = None;
    Ok(ack())
}

pub(super) async fn health(
    session: &super::super::BrowserSession,
) -> Result<BrowserOutput, BrowserError> {
    let native = session.inner.native.lock().await;
    let Some(runtime) = native.as_ref() else {
        return Ok(BrowserOutput::Json(json!({
            "ok": true,
            "backend": "tetherscript-native",
            "started": false
        })));
    };
    Ok(BrowserOutput::Json(json!({
        "ok": true,
        "alive": true,
        "backend": "tetherscript-native",
        "current": runtime.current,
        "started": true,
        "tabs": runtime.pages.len()
    })))
}

pub(super) fn ack() -> BrowserOutput {
    BrowserOutput::Ack(Ack { ok: true })
}
