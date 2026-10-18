//! Typed inputs for proactive session-context preparation.

use std::sync::Arc;

use codetether_context_policy::summary_index::SummaryIndex;
use codetether_provider_core::{Message, Provider};
use codetether_rlm::RlmConfig;

use codetether_proactive_planning::Slot;

/// Provider, model and configuration registered for proactive work.
/// # Examples
/// ```
/// # fn example(provider: std::sync::Arc<dyn codetether_provider_core::Provider>) {
/// let runtime = codetether_proactive_runtime::Runtime { provider, model: "model".into(), config: Default::default() };
/// assert_eq!(runtime.model, "model");
/// # }
/// ```
#[derive(Clone)]
pub struct Runtime {
    pub provider: Arc<dyn Provider>,
    pub model: String,
    pub config: RlmConfig,
}

/// Coalesced session input tagged with the current registry generation.
/// # Examples
/// ```
/// # fn example(runtime: codetether_proactive_runtime::Runtime) {
/// let snapshot = codetether_proactive_runtime::Snapshot { session_id: "session".into(), messages: Vec::new(), index: Default::default(), runtime, generation: 1 };
/// assert_eq!(snapshot.generation, 1);
/// # }
/// ```
#[derive(Clone)]
pub struct Snapshot {
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
