//! Heartbeat publication uses weak cache/reader references, never another LSP.
use codetether_diagnostics_bridge::{Diagnostics, Publisher};
use std::sync::{
    Arc, Mutex, Weak,
    atomic::{AtomicBool, Ordering},
};
use tokio::sync::RwLock;

pub(super) fn start(
    cache: Weak<RwLock<Diagnostics>>,
    alive: Weak<AtomicBool>,
    publisher: Arc<Mutex<Option<Publisher>>>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(2));
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        let mut reported_error = false;
        loop {
            interval.tick().await;
            let Some(cache) = cache.upgrade() else { break };
            if !alive
                .upgrade()
                .is_some_and(|alive| alive.load(Ordering::SeqCst))
            {
                break;
            }
            let snapshot = cache.read().await.clone();
            drop(cache);
            let publisher = publisher.lock().unwrap_or_else(|error| error.into_inner());
            let Some(publisher) = publisher.as_ref() else {
                break;
            };
            if let Err(error) = publisher.publish(&snapshot) {
                if !reported_error {
                    tracing::warn!(error = %error, "Diagnostic snapshot write failed");
                    reported_error = true;
                }
            } else {
                reported_error = false;
            }
        }
        publisher
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .take();
    })
}
