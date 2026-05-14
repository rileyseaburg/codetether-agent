use crate::browser::{BrowserError, BrowserOutput, output::HtmlContent, request::ScopeRequest};

pub(super) async fn html(
    session: &super::super::super::BrowserSession,
    request: ScopeRequest,
) -> Result<BrowserOutput, BrowserError> {
    let mut native = session.inner.native.lock().await;
    let slot = native
        .as_mut()
        .ok_or(BrowserError::SessionNotStarted)?
        .current_mut()?;
    let mut page = slot.page();
    let html = match request.selector {
        Some(selector) => super::super::eval::string(
            &mut page,
            &format!("document.querySelector({}).outerHTML", quote(&selector)),
        )?,
        None => page.session.html.clone(),
    };
    slot.replace(page);
    Ok(BrowserOutput::Html(HtmlContent { html }))
}

fn quote(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "\"\"".into())
}
