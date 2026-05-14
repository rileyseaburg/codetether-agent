use crate::browser::{BrowserError, BrowserOutput, output::TextContent, request::ScopeRequest};
use tetherscript::browser::{query_selector, text_content};

pub(in crate::browser::session::native) async fn text(
    session: &super::super::super::BrowserSession,
    request: ScopeRequest,
) -> Result<BrowserOutput, BrowserError> {
    let native = session.inner.native.lock().await;
    let page = native
        .as_ref()
        .ok_or(BrowserError::SessionNotStarted)?
        .current()?;
    let text = request
        .selector
        .as_deref()
        .and_then(|selector| {
            query_selector(&page.session.document, selector)
                .first()
                .cloned()
        })
        .map(|node| text_content(&node))
        .unwrap_or_else(|| super::title::document_text(page));
    Ok(BrowserOutput::Text(TextContent { text }))
}
