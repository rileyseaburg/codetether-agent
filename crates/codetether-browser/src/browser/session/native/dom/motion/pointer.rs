//! Pointer movement operations.

use crate::browser::{BrowserError, BrowserOutput, request::SelectorRequest};

/// Click an element by selector.
///
/// # Errors
///
/// Returns [`BrowserError`] when the session is not started or click handling
/// fails.
pub(in crate::browser::session::native) async fn click(
    session: &super::super::super::super::BrowserSession,
    request: SelectorRequest,
) -> Result<BrowserOutput, BrowserError> {
    super::super::page::with(session, |page| {
        page.click(&super::super::selector::css(request.selector))
            .map(|_| ())
    })
    .await
}

/// Hover an element by selector.
///
/// # Errors
///
/// Returns [`BrowserError`] when the session is not started or hover handling
/// fails.
pub(in crate::browser::session::native) async fn hover(
    session: &super::super::super::super::BrowserSession,
    request: SelectorRequest,
) -> Result<BrowserOutput, BrowserError> {
    super::super::page::with(session, |page| {
        page.hover(&super::super::selector::css(request.selector))
            .map(|_| ())
    })
    .await
}
