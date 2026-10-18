//! Native session adapter for shared ledger construction.
use crate::session::Session;
pub(crate) use codetether_evidence_ledger::ledger::{ScopeLedger, unclassified_count};
pub(crate) fn build(session: &Session) -> ScopeLedger {
    codetether_evidence_ledger::ledger::build(&session.id, &session.messages)
}
