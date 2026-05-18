//! Focus and blur operations.

use crate::browser::{BrowserError, BrowserOutput, request::SelectorRequest};

/// Focus an element by selector.
///
/// # Errors
///
/// Returns [`BrowserError`] when the session is not started or JavaScript
/// evaluation fails.
pub(in crate::browser::session::native) async fn focus(
    session: &super::super::super::super::BrowserSession,
    request: SelectorRequest,
) -> Result<BrowserOutput, BrowserError> {
    run(session, &request.selector, "focus").await
}

/// Blur an element by selector.
///
/// # Errors
///
/// Returns [`BrowserError`] when the session is not started or JavaScript
/// evaluation fails.
pub(in crate::browser::session::native) async fn blur(
    session: &super::super::super::super::BrowserSession,
    request: SelectorRequest,
) -> Result<BrowserOutput, BrowserError> {
    run(session, &request.selector, "blur").await
}

async fn run(
    session: &super::super::super::super::BrowserSession,
    selector: &str,
    action: &str,
) -> Result<BrowserOutput, BrowserError> {
    let script = format!(
        "document.querySelector({}).{}()",
        super::super::selector::quote(selector),
        action
    );
    super::super::page::with(session, |page| page.eval_js(&script).map(|_| ())).await
}
