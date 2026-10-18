//! Native session identifier adapter for shared evidence writeback.
use crate::session::Session;
pub(crate) fn save(session: &Session, ledger: &super::ledger::ScopeLedger) {
    codetether_evidence_ledger::writeback_persist::save(&session.id, ledger);
}
