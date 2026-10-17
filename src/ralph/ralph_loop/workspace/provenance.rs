//! commit_provenance application policy and provenance integration.
use super::*;
impl RalphLoop {
    pub(in crate::ralph::ralph_loop) fn commit_provenance(&self) -> ExecutionProvenance {
        let mut provenance = ExecutionProvenance::for_operation("ralph", ExecutionOrigin::Ralph);
        provenance.set_run_id(self.run_id.clone());
        provenance
    }
}
