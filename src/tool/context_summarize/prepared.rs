//! Owned snapshot of the inputs used by the existing summary producer.
use crate::provider::{Message, Provider};
use std::sync::Arc;
/// Session producer inputs, captured before mutable cache access.
pub struct Prepared {
    pub(super) messages: crate::session::tracked::TrackedVec<Message>,
    pub(super) generation: u64,
    pub(super) rlm: crate::rlm::RlmConfig,
    pub(super) session_id: String,
    pub(super) sub_provider: Option<Arc<dyn Provider>>,
    pub(super) sub_model: Option<String>,
}
pub(super) fn capture(session: &crate::session::Session) -> Prepared {
    let messages = session.messages.clone();
    let generation = session.summary_index.generation();
    let rlm = session.metadata.rlm.clone();
    let session_id = session.id.clone();
    let sub_provider = session.metadata.subcall_provider.clone();
    let sub_model = session.metadata.subcall_model_name.clone();

    Prepared {
        messages,
        generation,
        rlm,
        session_id,
        sub_provider,
        sub_model,
    }
}
