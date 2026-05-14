use crate::browser::{BrowserError, BrowserOutput, output::Ack, request::NewTabRequest};

pub(in crate::browser::session::native) async fn new(
    session: &super::super::super::BrowserSession,
    request: NewTabRequest,
) -> Result<BrowserOutput, BrowserError> {
    let url = request.url.unwrap_or_else(|| "about:blank".into());
    let html = super::super::fetch::html(&url).await?;
    let mut native = session.inner.native.lock().await;
    let runtime = native.as_mut().ok_or(BrowserError::SessionNotStarted)?;
    runtime
        .pages
        .push(super::super::NativePage::from_html(url, html));
    runtime.current = runtime.pages.len() - 1;
    Ok(BrowserOutput::Ack(Ack { ok: true }))
}
