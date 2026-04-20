//! Replay HTTP requests via the page's `fetch()` API.

use super::super::{BrowserSession, access};
use crate::browser::{BrowserError, BrowserOutput, request::FetchRequest};
use std::time::Duration;

const EVAL_TIMEOUT: Duration = Duration::from_secs(60);

/// Execute an in-page `fetch()` with the given method, headers, and body.
///
/// Auto-inherits request headers (auth, CSRF, X-*) from the app's most
/// recent successful same-URL+method entry in `__codetether_net_log`,
/// and on `status: 0` / `Failed to fetch` attaches a `hint` +
/// `suggested_actions` naming the next transport to try (xhr / axios /
/// diagnose). See [`super::fallback_js`] for the shared JS.
///
/// # Errors
///
/// Returns [`BrowserError::EvaluationTimeout`] if the fetch exceeds 60 s,
/// or propagates CDP / serialization errors.
pub(super) async fn fetch(
    session: &BrowserSession,
    request: FetchRequest,
) -> Result<BrowserOutput, BrowserError> {
    let page = access::current_page(session).await?;
    let script = super::fetch_tmpl::build_script(request)?;
    let result = tokio::time::timeout(EVAL_TIMEOUT, page.evaluate_expression(script))
        .await
        .map_err(|_| BrowserError::EvaluationTimeout)??;
    let value = result.object().value.clone()
        .unwrap_or(serde_json::json!({"ok": false, "status": 0, "error": "no value"}));
    Ok(BrowserOutput::Json(value))
}
