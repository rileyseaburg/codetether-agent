//! Replay HTTP requests via raw `XMLHttpRequest`.
//!
//! Matches the app's own XHR wire bytes including `Sec-Fetch-*`
//! headers, cookie handling, and service-worker routing.

use super::super::{BrowserSession, access};
use crate::browser::{BrowserError, BrowserOutput, request::XhrRequest};
use std::time::Duration;

const EVAL_TIMEOUT: Duration = Duration::from_secs(60);

/// Replay via raw `XMLHttpRequest` so `Sec-Fetch-*`, cookies, and
/// service-worker routing match the app's own XHR save path.
///
/// # Arguments
///
/// * `session` — active browser session with a current page
/// * `request` — URL, method, headers, body, with_credentials flag
///
/// # Errors
///
/// Returns [`BrowserError::EvaluationTimeout`] if the XHR exceeds 60 s,
/// or propagates CDP / serialization errors.
pub async fn xhr(
    session: &BrowserSession,
    request: XhrRequest,
) -> Result<BrowserOutput, BrowserError> {
    let page = access::current_page(session).await?;
    let script = super::xhr_tmpl::build_script(request)?;
    let result = tokio::time::timeout(EVAL_TIMEOUT, page.evaluate_expression(script))
        .await
        .map_err(|_| BrowserError::EvaluationTimeout)??;
    let value = result.object().value.clone()
        .unwrap_or(serde_json::json!({"ok": false, "status": 0, "error": "no value"}));
    Ok(BrowserOutput::Json(value))
}
