//! Replay a previously-captured network request with optional body edits.
//!
//! Looks up the most recent entry in `window.__codetether_net_log` whose
//! URL contains the caller-supplied needle (and optionally matches a
//! method filter), inherits all request headers from that captured
//! entry (notably `Authorization`), optionally deep-merges a JSON patch
//! into the captured JSON body, and re-fires the request via raw XHR.
//!
//! This is the "I saw it happen once, now do it again with these
//! edits" primitive — use it instead of reconstructing a call from
//! scratch after capturing one real save via the UI.

use super::super::{BrowserSession, access};
use crate::browser::{BrowserError, BrowserOutput, request::ReplayRequest};
use std::time::Duration;

const EVAL_TIMEOUT: Duration = Duration::from_secs(60);

/// Replay a captured request; see module docs for semantics.
///
/// # Errors
///
/// Returns [`BrowserError::EvaluationTimeout`] after 60 s, or
/// propagates CDP / serialization errors from the page evaluation.
pub async fn replay(
    session: &BrowserSession,
    request: ReplayRequest,
) -> Result<BrowserOutput, BrowserError> {
    let page = access::current_page(session).await?;
    let _ = super::super::lifecycle::install_page_hooks(&page).await;
    let script = super::replay_tmpl::build_script(request)?;
    let result = tokio::time::timeout(EVAL_TIMEOUT, page.evaluate_expression(script))
        .await
        .map_err(|_| BrowserError::EvaluationTimeout)??;
    let value = result.object().value.clone()
        .unwrap_or(serde_json::json!({"ok": false, "status": 0, "error": "no value"}));
    Ok(BrowserOutput::Json(value))
}
