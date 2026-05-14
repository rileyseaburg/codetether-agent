//! Form-oriented DOM operations.

use crate::browser::{
    BrowserError, BrowserOutput,
    request::{ClickTextRequest, FillRequest, ToggleRequest},
};
use tetherscript::browser_agent::Locator;

/// Fill an input-like element by selector.
///
/// # Errors
///
/// Returns [`BrowserError`] when the session is not started, the selector does
/// not resolve, or the element cannot be filled.
pub(in crate::browser::session::native) async fn fill(
    session: &super::super::super::BrowserSession,
    request: FillRequest,
) -> Result<BrowserOutput, BrowserError> {
    super::page::with(session, |page| {
        page.fill(&super::selector::css(request.selector), &request.value)
            .map(|_| ())
    })
    .await
}

/// Click an element matched by visible text.
///
/// # Errors
///
/// Returns [`BrowserError`] when the session is not started or no matching
/// element can be clicked.
pub(in crate::browser::session::native) async fn click_text(
    session: &super::super::super::BrowserSession,
    request: ClickTextRequest,
) -> Result<BrowserOutput, BrowserError> {
    let locator = if request.exact {
        Locator::text_exact(request.text).relaxed()
    } else {
        Locator::text(request.text).relaxed()
    };
    super::page::with(session, |page| page.click(&locator).map(|_| ())).await
}

/// Toggle a checkbox-like control by selector.
///
/// # Errors
///
/// Returns [`BrowserError`] when the session is not started or script
/// evaluation fails.
pub(in crate::browser::session::native) async fn toggle(
    session: &super::super::super::BrowserSession,
    request: ToggleRequest,
) -> Result<BrowserOutput, BrowserError> {
    let script = format!(
        "let el=document.querySelector({});el.checked=!el.checked;el.dispatchEvent({{type:'change'}});",
        super::selector::quote(&request.selector)
    );
    super::page::with(session, |page| page.eval_js(&script).map(|_| ())).await
}
