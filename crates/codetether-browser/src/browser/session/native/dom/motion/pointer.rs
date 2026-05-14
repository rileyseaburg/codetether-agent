use crate::browser::{BrowserError, BrowserOutput, request::SelectorRequest};

pub(super) async fn click(
    session: &super::super::super::super::BrowserSession,
    request: SelectorRequest,
) -> Result<BrowserOutput, BrowserError> {
    super::super::page::with(session, |page| {
        page.click(&super::super::selector::css(request.selector))
            .map(|_| ())
    })
    .await
}

pub(super) async fn hover(
    session: &super::super::super::super::BrowserSession,
    request: SelectorRequest,
) -> Result<BrowserOutput, BrowserError> {
    super::super::page::with(session, |page| {
        page.hover(&super::super::selector::css(request.selector))
            .map(|_| ())
    })
    .await
}
