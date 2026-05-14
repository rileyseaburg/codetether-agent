use crate::browser::{BrowserError, BrowserOutput, request::NetworkLogRequest};
use serde_json::json;

pub(super) async fn log(
    session: &super::super::super::super::BrowserSession,
    request: NetworkLogRequest,
) -> Result<BrowserOutput, BrowserError> {
    let native = session.inner.native.lock().await;
    let page = native
        .as_ref()
        .ok_or(BrowserError::SessionNotStarted)?
        .current()?;
    let method = request.method.map(|value| value.to_uppercase());
    let limit = request.limit.unwrap_or(usize::MAX);
    let items: Vec<_> = page
        .session
        .network
        .iter()
        .filter(|event| matches(event, method.as_deref(), request.url_contains.as_deref()))
        .take(limit)
        .map(|event| {
            json!({"method": event.method.clone(), "url": event.url.clone(), "status": event.status})
        })
        .collect();
    Ok(BrowserOutput::Json(json!(items)))
}

fn matches(
    event: &tetherscript::browser_session::NetworkEvent,
    method: Option<&str>,
    needle: Option<&str>,
) -> bool {
    method.is_none_or(|want| event.method == want)
        && needle.is_none_or(|value| event.url.contains(value))
}
