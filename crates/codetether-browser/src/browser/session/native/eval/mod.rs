//! JavaScript evaluation for native browser pages.

mod value;

use crate::browser::{BrowserError, BrowserOutput, output::EvalOutput, request::EvalRequest};
use tetherscript::browser_agent::BrowserPage;

/// Evaluate JavaScript in the current page.
///
/// # Errors
///
/// Returns [`BrowserError`] when the session is not started or evaluation
/// fails.
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

/// Evaluate JavaScript and return the display string for the result.
///
/// # Errors
///
/// Returns [`BrowserError`] when evaluation fails.
pub(super) fn string(page: &mut BrowserPage, script: &str) -> Result<String, BrowserError> {
    Ok(page.eval_js(script).map_err(js_error)?.display())
}

/// Convert a TetherScript JavaScript error string into a browser error.
pub(super) fn js_error(message: String) -> BrowserError {
    BrowserError::JsException {
        message,
        stack: None,
    }
}
