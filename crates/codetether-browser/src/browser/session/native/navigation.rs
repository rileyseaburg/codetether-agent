use crate::browser::{BrowserError, BrowserOutput, request::NavigationRequest};

pub(super) async fn goto(
    session: &super::super::BrowserSession,
    request: NavigationRequest,
) -> Result<BrowserOutput, BrowserError> {
    let html = super::fetch::html(&request.url).await?;
    let mut native = session.inner.native.lock().await;
    let slot = native
        .as_mut()
        .ok_or(BrowserError::SessionNotStarted)?
        .current_mut()?;
    let mut page = slot.page();
    page.goto_html(request.url, html);
    let _ = page.run_scripts();
    slot.replace(page);
    Ok(super::lifecycle::ack())
}

pub(super) async fn back(
    session: &super::super::BrowserSession,
) -> Result<BrowserOutput, BrowserError> {
    let mut native = session.inner.native.lock().await;
    native
        .as_mut()
        .ok_or(BrowserError::SessionNotStarted)?
        .current_mut()?
        .session
        .back();
    Ok(super::lifecycle::ack())
}

pub(super) async fn reload(
    session: &super::super::BrowserSession,
) -> Result<BrowserOutput, BrowserError> {
    let mut native = session.inner.native.lock().await;
    let slot = native
        .as_mut()
        .ok_or(BrowserError::SessionNotStarted)?
        .current_mut()?;
    let mut page = slot.page();
    page.reload();
    let _ = page.run_scripts();
    slot.replace(page);
    Ok(super::lifecycle::ack())
}
