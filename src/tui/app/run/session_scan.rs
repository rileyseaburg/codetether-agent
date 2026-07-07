use std::path::Path;
use std::time::Duration;

use crate::session::{Session, TailLoad};
use crate::tui::app::resume_window::session_resume_window;

/// Default budget for the startup session scan before falling back to a fresh
/// session. Override with `CODETETHER_SESSION_SCAN_TIMEOUT_SECS`.
const DEFAULT_SCAN_BUDGET_SECS: u64 = 3;

/// Resolve the scan budget, honouring `CODETETHER_SESSION_SCAN_TIMEOUT_SECS`.
/// Values are clamped to at least 1 second; a value of 0 or an unparseable
/// value falls back to the default.
fn scan_budget() -> Duration {
    let secs = std::env::var("CODETETHER_SESSION_SCAN_TIMEOUT_SECS")
        .ok()
        .and_then(|raw| raw.parse::<u64>().ok())
        .filter(|&s| s > 0)
        .unwrap_or(DEFAULT_SCAN_BUDGET_SECS);
    Duration::from_secs(secs)
}

pub(super) async fn load(cwd: &Path) -> anyhow::Result<TailLoad> {
    if std::env::var_os("CODETETHER_TUI_NEW_SESSION").is_some() {
        return Err(anyhow::anyhow!("requested fresh session"));
    }
    let budget = scan_budget();
    match tokio::time::timeout(budget, scan(cwd)).await {
        Ok(result) => result,
        Err(_) => {
            tracing::warn!(
                budget_secs = budget.as_secs(),
                "session scan exceeded budget; starting fresh session"
            );
            Err(anyhow::anyhow!(
                "session scan timed out after {}s",
                budget.as_secs()
            ))
        }
    }
}

async fn scan(cwd: &Path) -> anyhow::Result<TailLoad> {
    Session::last_for_directory_tail(Some(cwd), session_resume_window()).await
}
