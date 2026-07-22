//! Recursive-language-model routing for large tool output.

use super::{super::Runner, call::Call};
use crate::session::SessionEvent;
use std::sync::Arc;
use tokio::sync::mpsc;

/// Routes oversized tool output through the configured recursive model.
pub(super) fn route(runner: &Runner<'_>, call: &Call, content: &str) -> String {
    let notify = runner.events.as_ref().map(notifier);
    super::super::super::rlm_background::route_or_defer(
        content,
        &call.name,
        &call.input,
        &call.id,
        &runner.session.id,
        &runner.session.messages,
        &runner.model.model_id,
        runner.model.provider.clone(),
        &runner.session.metadata.rlm,
        notify,
    )
}

fn notifier(tx: &mpsc::Sender<SessionEvent>) -> super::super::super::rlm_background::Notify {
    let tx = tx.downgrade();
    Arc::new(move |event| {
        if let Some(tx) = tx.upgrade() {
            let _ = tx.try_send(event);
        }
    })
}

#[cfg(test)]
#[path = "rlm_tests.rs"]
mod tests;
