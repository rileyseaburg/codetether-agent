//! Native session specialization of shared dirty-suffix encoding.
use super::state::Checkpoint;
use crate::session::Session;
use anyhow::Result;
pub(super) use codetether_session_store::Batch;
pub(super) fn prepare(session: &Session, checkpoint: &Checkpoint) -> Result<Batch> {
    codetether_session_store::prepare(session, checkpoint)
}
