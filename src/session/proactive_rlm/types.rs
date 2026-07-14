//! Typed inputs for proactive session-context preparation.

use std::sync::Arc;

use crate::provider::{Message, Provider};
use crate::rlm::RlmConfig;
use crate::session::index::SummaryIndex;

use super::slot::Slot;

#[derive(Clone)]
pub(super) struct Runtime {
    pub provider: Arc<dyn Provider>,
    pub model: String,
    pub config: RlmConfig,
}

#[derive(Clone)]
pub(super) struct Snapshot {
    pub session_id: String,
    pub messages: Vec<Message>,
    pub index: SummaryIndex,
    pub runtime: Runtime,
    pub generation: u64,
}

pub(super) struct State {
    pub runtime: Runtime,
    pub slot: Slot<Snapshot>,
}

impl State {
    pub fn new(runtime: Runtime) -> Self {
        Self {
            runtime,
            slot: Slot::default(),
        }
    }
}
