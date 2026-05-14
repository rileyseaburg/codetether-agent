use crate::browser::{BrowserError, BrowserOutput, request::ScrollRequest};

pub(in crate::browser::session::native) async fn scroll(
    session: &super::super::super::super::BrowserSession,
    request: ScrollRequest,
) -> Result<BrowserOutput, BrowserError> {
    let dx = request.x.unwrap_or_default() as i64;
    let dy = request.y.unwrap_or_default() as i64;
    super::super::page::with(session, |page| {
        page.session.scroll.x = page.session.scroll.x.saturating_add(dx).max(0);
        page.session.scroll.y = page.session.scroll.y.saturating_add(dy).max(0);
        Ok(())
    })
    .await
}
