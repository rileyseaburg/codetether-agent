use std::path::Path;
use std::time::Duration;

use crate::session::{Session, TailLoad};
use crate::tui::app::resume_window::session_resume_window;

const SESSION_SCAN_BUDGET: Duration = Duration::from_secs(3);

pub(super) async fn load(cwd: &Path) -> anyhow::Result<TailLoad> {
    if std::env::var_os("CODETETHER_TUI_NEW_SESSION").is_some() {
        return Err(anyhow::anyhow!("requested fresh session"));
    }
    match tokio::time::timeout(SESSION_SCAN_BUDGET, scan(cwd)).await {
        Ok(result) => result,
        Err(_) => {
            tracing::warn!(
                budget_secs = SESSION_SCAN_BUDGET.as_secs(),
                "session scan exceeded budget; starting fresh session"
            );
            Err(anyhow::anyhow!(
                "session scan timed out after {}s",
                SESSION_SCAN_BUDGET.as_secs()
            ))
        }
    }
}

async fn scan(cwd: &Path) -> anyhow::Result<TailLoad> {
    Session::last_for_directory_tail(Some(cwd), session_resume_window()).await
}
