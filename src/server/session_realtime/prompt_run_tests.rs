//! Prompt cancellation must finish dropping owned resources before returning.
use super::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
struct CleanupProbe(Arc<AtomicBool>);
impl Drop for CleanupProbe {
    fn drop(&mut self) {
        self.0.store(true, Ordering::SeqCst);
    }
}
#[tokio::test]
async fn cancel_waits_for_prompt_cleanup() {
    let cleaned = Arc::new(AtomicBool::new(false));
    let probe = CleanupProbe(Arc::clone(&cleaned));
    let (_event_tx, events) = mpsc::channel(1);
    let task = tokio::spawn(async move {
        let _probe = probe;
        std::future::pending::<Result<SessionResult, String>>().await
    });
    let mut run = PromptRun { events, task };
    run.cancel().await;
    assert!(cleaned.load(Ordering::SeqCst));
}
