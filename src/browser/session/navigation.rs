use super::{BrowserSession, access};
use crate::browser::{BrowserError, BrowserOutput, output::Ack, request::NavigationRequest};

pub(super) async fn goto(
    session: &BrowserSession,
    request: NavigationRequest,
) -> Result<BrowserOutput, BrowserError> {
    let page = access::current_page(session).await?;
    page.goto(request.url).await.map_err(map_error)?;
    apply_wait_until(&page, &request.wait_until).await?;
    Ok(BrowserOutput::Ack(Ack { ok: true }))
}

pub(super) async fn back(session: &BrowserSession) -> Result<BrowserOutput, BrowserError> {
    history_step(session, -1).await
}

pub(super) async fn reload(session: &BrowserSession) -> Result<BrowserOutput, BrowserError> {
    let page = access::current_page(session).await?;
    page.reload().await.map_err(map_error)?;
    Ok(BrowserOutput::Ack(Ack { ok: true }))
}

/// Navigate one entry forward (`+1`) or backward (`-1`) in the page history.
/// Errors with [`BrowserError::OperationFailed`] if the requested entry is out
/// of bounds (e.g. calling `back` on the first page in the session).
async fn history_step(session: &BrowserSession, delta: i64) -> Result<BrowserOutput, BrowserError> {
    use chromiumoxide::cdp::browser_protocol::page::{
        GetNavigationHistoryParams, NavigateToHistoryEntryParams,
    };
    let page = access::current_page(session).await?;
    let history = page.execute(GetNavigationHistoryParams::default()).await?;
    let target_index = history.current_index + delta;
    if target_index < 0 || target_index as usize >= history.entries.len() {
        return Err(BrowserError::OperationFailed(format!(
            "no history entry at offset {delta} (current_index={}, entries={})",
            history.current_index,
            history.entries.len()
        )));
    }
    let entry = &history.entries[target_index as usize];
    page.execute(NavigateToHistoryEntryParams::new(entry.id))
        .await?;
    Ok(BrowserOutput::Ack(Ack { ok: true }))
}

/// Honor the `wait_until` strategy chosen by the caller.
///
/// `chromiumoxide::Page::goto` returns at *commit* time (the moment the URL is
/// accepted), so any stricter wait must be implemented here.
async fn apply_wait_until(
    page: &chromiumoxide::page::Page,
    wait_until: &str,
) -> Result<(), BrowserError> {
    match wait_until.trim().to_ascii_lowercase().as_str() {
        // Default / fastest: return as soon as the browser has committed to
        // the new URL. Subresources and scripts may still be loading.
        "" | "commit" => Ok(()),
        // Wait until DOM is parsed (interactive) but subresources may still
        // be loading. Faster than `load` on ad/font-heavy pages.
        "domcontentloaded" | "dom" | "interactive" => {
            wait_for_ready_state(page, &["interactive", "complete"]).await
        }
        // Wait for the `load` event — all subresources finished.
        //
        // NOTE: we poll `document.readyState` rather than calling
        // `page.wait_for_navigation()` because the latter races: by the time
        // we'd call it, `page.goto()` has already returned (it only waits for
        // commit), and the navigation lifecycle may have already completed,
        // causing `wait_for_navigation` to hang waiting for the *next*
        // navigation that never comes.
        "load" | "complete" => wait_for_ready_state(page, &["complete"]).await,
        other => Err(BrowserError::OperationFailed(format!(
            "unknown wait_until value `{other}`; expected one of: commit, domcontentloaded, load"
        ))),
    }
}

/// Poll `document.readyState` until it matches one of the accepted values or
/// the 30 s budget elapses. Mirrors Puppeteer's `domcontentloaded` / `load`
/// semantics without hooking into chromiumoxide's internal lifecycle machinery
/// (which has a race where the event can fire between `goto` returning and
/// `wait_for_navigation` being polled).
async fn wait_for_ready_state(
    page: &chromiumoxide::page::Page,
    accept: &[&str],
) -> Result<(), BrowserError> {
    use tokio::time::{Duration, Instant, sleep};
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        // `evaluate` returns `EvaluationResult`. `into_value::<String>()`
        // returns `Result<String, serde_json::Error>`. We treat a deser
        // failure as "not ready yet" (empty string) so the outer timeout
        // still catches pathological cases rather than propagating a
        // confusing serde error as a navigation failure.
        let state = page
            .evaluate("document.readyState")
            .await
            .map_err(map_error)?
            .into_value::<String>()
            .unwrap_or_default();
        if accept.iter().any(|want| state == *want) {
            return Ok(());
        }
        if Instant::now() >= deadline {
            return Err(BrowserError::NavigationTimeout);
        }
        sleep(Duration::from_millis(25)).await;
    }
}

fn map_error(error: chromiumoxide::error::CdpError) -> BrowserError {
    match error {
        chromiumoxide::error::CdpError::Timeout => BrowserError::NavigationTimeout,
        other => other.into(),
    }
}
