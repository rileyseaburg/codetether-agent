//! Optional play break shown by interactive clients during a delayed retry.

use super::Host;
use codetether_session_events::SessionEvent;

pub(super) async fn wait(host: &mut impl Host, seconds: u64) {
    emit(host, true).await;
    tokio::time::sleep(std::time::Duration::from_secs(seconds)).await;
    emit(host, false).await;
}

async fn emit(host: &mut impl Host, active: bool) {
    if let Some(events) = host.events() {
        let _ = events.send(SessionEvent::PlayBreak(active)).await;
    }
}
