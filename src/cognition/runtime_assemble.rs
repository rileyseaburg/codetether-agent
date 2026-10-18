//! Bind concrete application service types to shared runtime state assembly.
use super::{
    CognitionRuntime, CognitionRuntimeOptions, ThinkerClient, ThoughtEvent,
    initial_state::InitialState,
};
use std::sync::Arc;
use tokio::sync::broadcast;
pub(super) fn assemble(
    options: CognitionRuntimeOptions,
    init: InitialState,
    thinker: Option<Arc<ThinkerClient>>,
    event_tx: broadcast::Sender<ThoughtEvent>,
) -> CognitionRuntime {
    CognitionRuntime {
        state: codetether_cognition_core::runtime::CognitionRuntime::assemble(
            options, init, thinker, event_tx,
        ),
    }
}
