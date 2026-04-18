use super::{BrowserSession, access::RuntimeAccess};
use crate::browser::BrowserOutput;
use serde_json::json;

pub(super) async fn run(
    session: &BrowserSession,
) -> Result<BrowserOutput, crate::browser::BrowserError> {
    let Some((alive, runtime)) = state(session).await else {
        return Ok(BrowserOutput::Json(json!({"ok": true, "started": false})));
    };
    let ws_url = runtime.browser.lock().await.websocket_address().clone();
    Ok(BrowserOutput::Json(json!({
        "ok": true,
        "alive": alive,
        "mode": runtime.mode.as_str(),
        "started": true,
        "ws_url": ws_url
    })))
}

async fn state(session: &BrowserSession) -> Option<(bool, RuntimeAccess)> {
    let slot = session.inner.runtime.lock().await;
    let runtime = slot.as_ref()?;
    Some((runtime.is_alive(), RuntimeAccess::from(runtime)))
}
