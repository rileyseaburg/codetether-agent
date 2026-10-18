//! commit_provenance application policy and provenance integration.
use super::*;
pub(crate) fn commit_provenance(run_id: &str) -> ExecutionProvenance {
    let mut provenance = ExecutionProvenance::for_operation("ralph", ExecutionOrigin::Ralph);
    provenance.set_run_id(run_id.to_owned());
    provenance
}
