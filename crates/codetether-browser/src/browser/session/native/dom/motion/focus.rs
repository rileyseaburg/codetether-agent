use crate::browser::{BrowserError, BrowserOutput, request::SelectorRequest};

pub(super) async fn focus(
    session: &super::super::super::super::BrowserSession,
    request: SelectorRequest,
) -> Result<BrowserOutput, BrowserError> {
    run(session, &request.selector, "focus").await
}

pub(super) async fn blur(
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
