use crate::browser::{
    BrowserError, BrowserOutput,
    request::{ClickTextRequest, FillRequest, ToggleRequest},
};
use tetherscript::browser_agent::Locator;

pub(super) async fn fill(
    session: &super::super::super::BrowserSession,
    request: FillRequest,
) -> Result<BrowserOutput, BrowserError> {
    super::page::with(session, |page| {
        page.fill(&super::selector::css(request.selector), &request.value)
            .map(|_| ())
    })
    .await
}

pub(super) async fn click_text(
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

pub(super) async fn toggle(
    session: &super::super::super::BrowserSession,
    request: ToggleRequest,
) -> Result<BrowserOutput, BrowserError> {
    let script = format!(
        "let el=document.querySelector({});el.checked=!el.checked;el.dispatchEvent({{type:'change'}});",
        super::selector::quote(&request.selector)
    );
    super::page::with(session, |page| page.eval_js(&script).map(|_| ())).await
}
