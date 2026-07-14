//! Recursive-language-model routing for large tool output.

use super::{super::Runner, call::Call};
use std::sync::Arc;

/// Routes oversized tool output through the configured recursive model.
pub(super) fn route(runner: &Runner<'_>, call: &Call, content: &str) -> String {
    let notify = runner.events.as_ref().map(|tx| {
        let tx = tx.clone();
        Arc::new(move |event| {
            let _ = tx.try_send(event);
        }) as super::super::super::rlm_background::Notify
    });
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
