//! Sync status callbacks for background RLM.

use std::sync::Arc;
use uuid::Uuid;

use codetether_rlm::events::RlmProgressEvent;

/// Receives background progress and terminal status as RLM progress payloads.
pub type Notify = Arc<dyn Fn(RlmProgressEvent) + Send + Sync>;

pub(super) fn progress(notify: &Option<Notify>, status: &str, iteration: usize, max: usize) {
    emit(
        notify,
        RlmProgressEvent {
            trace_id: Uuid::new_v4(),
            iteration,
            max_iterations: max,
            status: status.into(),
        },
    );
}

pub(super) fn complete(notify: &Option<Notify>, ok: bool, reason: Option<String>) {
    let status = if ok {
        "background ready".into()
    } else {
        format!(
            "background failed: {}",
            reason.unwrap_or_else(|| "unknown".into())
        )
    };
    progress(notify, &status, 1, 1);
}

fn emit(notify: &Option<Notify>, event: RlmProgressEvent) {
    if let Some(notify) = notify {
        notify(event);
    }
}
