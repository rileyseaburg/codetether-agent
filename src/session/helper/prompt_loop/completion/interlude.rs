//! Optional play break shown by interactive clients during a delayed retry.

use super::Runner;
use crate::session::SessionEvent;

pub(super) async fn wait(runner: &Runner<'_>, seconds: u64) {
    emit(runner, true).await;
    tokio::time::sleep(std::time::Duration::from_secs(seconds)).await;
    emit(runner, false).await;
}

async fn emit(runner: &Runner<'_>, active: bool) {
    if let Some(events) = runner.events.as_ref() {
        let _ = events.send(SessionEvent::PlayBreak(active)).await;
    }
}
