//! Text and key input operations.

use crate::browser::{
    BrowserError, BrowserOutput,
    request::{KeyPressRequest, TypeRequest},
};

/// Type text through a selector-targeted page operation.
///
/// # Errors
///
/// Returns [`BrowserError`] when the session is not started or script
/// evaluation fails.
pub(in crate::browser::session::native) async fn type_text(
    session: &super::super::super::BrowserSession,
    request: TypeRequest,
) -> Result<BrowserOutput, BrowserError> {
    let script = format!(
        "let el=document.querySelector({});el.value=(el.value||'')+{};el.dispatchEvent({{type:'input'}});",
        super::selector::quote(&request.selector),
        super::selector::quote(&request.text)
    );
    super::page::with(session, |page| page.eval_js(&script).map(|_| ())).await
}

/// Press a keyboard key through the page input model.
///
/// # Errors
///
/// Returns [`BrowserError`] when the session is not started or input fails.
pub(in crate::browser::session::native) async fn press(
    session: &super::super::super::BrowserSession,
    request: KeyPressRequest,
) -> Result<BrowserOutput, BrowserError> {
    super::page::with(session, |page| {
        page.press(&super::selector::css(request.selector), request.key)
            .map(|_| ())
    })
    .await
}
