mod value;

use crate::browser::{BrowserError, BrowserOutput, output::EvalOutput, request::EvalRequest};
use tetherscript::browser_agent::BrowserPage;

pub(super) async fn run(
    session: &super::super::BrowserSession,
    request: EvalRequest,
) -> Result<BrowserOutput, BrowserError> {
    let mut native = session.inner.native.lock().await;
    let page = native
        .as_mut()
        .ok_or(BrowserError::SessionNotStarted)?
        .current_mut()?;
    let mut browser_page = page.page();
    let result = browser_page
        .eval_js(&request.expression)
        .map_err(js_error)?;
    page.replace(browser_page);
    Ok(BrowserOutput::Eval(EvalOutput {
        result: value::to_json(&result),
    }))
}

pub(super) fn string(page: &mut BrowserPage, script: &str) -> Result<String, BrowserError> {
    Ok(page.eval_js(script).map_err(js_error)?.display())
}

pub(super) fn js_error(message: String) -> BrowserError {
    BrowserError::JsException {
        message,
        stack: None,
    }
}
