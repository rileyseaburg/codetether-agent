//! Replay HTTP requests through the page's own axios instance.
//!
//! Inherits interceptors, auth headers, CSRF tokens, and baseURL
//! from the live page's axios instance.

use super::super::{BrowserSession, access};
use crate::browser::{BrowserError, BrowserOutput, request::AxiosRequest};
use std::time::Duration;

const EVAL_TIMEOUT: Duration = Duration::from_secs(60);

/// Replay through the page's own axios instance so interceptors (auth,
/// CSRF, baseURL, request IDs) all apply.
///
/// # Arguments
///
/// * `session` — active browser session with a current page
/// * `request` — URL, method, headers, body, optional axios_path
///
/// # Errors
///
/// Returns [`BrowserError::EvaluationTimeout`] if the call exceeds 60 s,
/// or propagates CDP / serialization errors.
pub async fn axios(
    session: &BrowserSession,
    request: AxiosRequest,
) -> Result<BrowserOutput, BrowserError> {
    let page = access::current_page(session).await?;
    let script = super::axios_tmpl::build_script(request)?;
    let result = tokio::time::timeout(EVAL_TIMEOUT, page.evaluate_expression(script))
        .await
        .map_err(|_| BrowserError::EvaluationTimeout)??;
    let value = result
        .object()
        .value
        .clone()
        .unwrap_or(serde_json::json!({"ok": false, "status": 0, "error": "no value"}));
    Ok(BrowserOutput::Json(value))
}
