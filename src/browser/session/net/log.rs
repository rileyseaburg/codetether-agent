//! Read the in-page network log captured by the injected hooks.

use super::super::{BrowserSession, access};
use crate::browser::{BrowserError, BrowserOutput, request::NetworkLogRequest};
use std::time::Duration;

const EVAL_TIMEOUT: Duration = Duration::from_secs(60);

/// Query `window.__codetether_net_log` with optional filtering.
///
/// # Arguments
///
/// * `session` — active browser session with a current page
/// * `request` — filter parameters (limit, url_contains, method)
///
/// # Errors
///
/// Returns [`BrowserError::EvaluationTimeout`] if the query exceeds 60 s,
/// or propagates CDP errors from the page evaluation.
pub async fn network_log(
    session: &BrowserSession,
    request: NetworkLogRequest,
) -> Result<BrowserOutput, BrowserError> {
    let page = access::current_page(session).await?;
    let _ = super::super::lifecycle::install_page_hooks(&page).await;
    let filter = serde_json::json!({
        "limit": request.limit,
        "url_contains": request.url_contains,
        "method": request.method.map(|m| m.to_uppercase()),
    });
    let script = SCRIPT.replace("__FILTER__", &filter.to_string());
    let result = tokio::time::timeout(EVAL_TIMEOUT, page.evaluate_expression(script))
        .await
        .map_err(|_| BrowserError::EvaluationTimeout)??;
    let value = result.object().value.clone().unwrap_or(serde_json::json!([]));
    Ok(BrowserOutput::Json(value))
}

const SCRIPT: &str = r#"(() => {
  const f = __FILTER__;
  const log = (window.__codetether_net_log || []).slice();
  const limit = typeof f.limit === 'number' && f.limit > 0 ? f.limit : log.length;
  const method = f.method || null;
  const needle = f.url_contains || null;
  const out = [];
  for (let i = log.length - 1; i >= 0 && out.length < limit; i--) {
    const e = log[i];
    if (method && e.method !== method) continue;
    if (needle && (!e.url || e.url.indexOf(needle) === -1)) continue;
    out.push(e);
  }
  return out.reverse();
})()"#;
