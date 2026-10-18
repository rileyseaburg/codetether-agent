//! Native checkpoint snapshot for shared manual-summary persistence.
use crate::session::{
    Session,
    index::{SummaryNode, SummaryRange},
};
use anyhow::Result;
pub(super) use codetether_session_store::summaries::read;
pub(crate) async fn put(session: &Session, range: SummaryRange, node: &SummaryNode) -> Result<()> {
    let state = session.storage.0.lock().unwrap().clone();
    codetether_session_store::summaries::put(state, range, node).await
}
