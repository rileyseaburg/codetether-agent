use crate::browser::{BrowserError, BrowserOutput, request::WaitRequest};
use tetherscript::browser::{query_selector, text_content};

pub(super) async fn run(
    session: &super::super::BrowserSession,
    request: WaitRequest,
) -> Result<BrowserOutput, BrowserError> {
    let native = session.inner.native.lock().await;
    let page = native
        .as_ref()
        .ok_or(BrowserError::SessionNotStarted)?
        .current()?;
    if let Some(selector) = &request.selector {
        let found = !query_selector(&page.session.document, selector).is_empty();
        return match (found, request.state.as_str()) {
            (true, _) | (false, "hidden" | "detached") => Ok(super::lifecycle::ack()),
            _ => Err(BrowserError::WaitTimeout {
                selector: selector.clone(),
                timeout_ms: request.timeout_ms,
            }),
        };
    }
    if let Some(text) = &request.text {
        let body = page
            .session
            .document
            .children
            .iter()
            .map(text_content)
            .collect::<String>();
        if body.contains(text) {
            return Ok(super::lifecycle::ack());
        }
    }
    Ok(super::lifecycle::ack())
}
