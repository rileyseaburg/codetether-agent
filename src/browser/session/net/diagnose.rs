//! Dump the page's HTTP plumbing for debugging.
//!
//! Reports service workers, discovered axios instances, document CSP,
//! and a summary of the in-page network log.

use super::super::{BrowserSession, access};
use crate::browser::{BrowserError, BrowserOutput, request::DiagnoseRequest};
use std::time::Duration;

const EVAL_TIMEOUT: Duration = Duration::from_secs(60);

/// Dump the page's HTTP plumbing: service workers, axios instances,
/// document CSP, and network-log summary.
///
/// # Arguments
///
/// * `session` — active browser session with a current page
/// * `_request` — placeholder; no fields currently used
///
/// # Errors
///
/// Returns [`BrowserError::EvaluationTimeout`] if the script exceeds 60 s,
/// or propagates CDP errors from the page evaluation.
pub async fn diagnose(
    session: &BrowserSession,
    _request: DiagnoseRequest,
) -> Result<BrowserOutput, BrowserError> {
    let page = access::current_page(session).await?;
    let _ = super::super::lifecycle::install_page_hooks(&page).await;
    let result = tokio::time::timeout(EVAL_TIMEOUT, page.evaluate_expression(super::diagnose_tmpl::SCRIPT))
        .await
        .map_err(|_| BrowserError::EvaluationTimeout)??;
    let value = result.object().value.clone().unwrap_or(serde_json::json!({}));
    Ok(BrowserOutput::Json(value))
}
