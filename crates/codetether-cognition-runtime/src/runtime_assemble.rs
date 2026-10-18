//! Bind concrete application service types to shared runtime state assembly.
use super::{CognitionRuntime, CognitionRuntimeOptions, InitialState, ThoughtEvent};
use crate::Host;
use std::sync::Arc;
use tokio::sync::broadcast;
pub(super) fn assemble<H: Host>(
    options: CognitionRuntimeOptions,
    init: InitialState,
    thinker: Option<Arc<H::Thinker>>,
    event_tx: broadcast::Sender<ThoughtEvent>,
) -> CognitionRuntime<H> {
    CognitionRuntime {
        state: codetether_cognition_core::runtime::CognitionRuntime::assemble(
            options, init, thinker, event_tx,
        ),
    }
}
