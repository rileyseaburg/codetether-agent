//! Application session metadata and provenance binding.
use crate::{
    provenance::ClaimProvenance,
    session::{Session, SessionMetadata},
};
impl codetether_a2a_worker_core::preparation::session::SessionState for Session {
    fn id(&self) -> &str {
        &self.id
    }
    fn metadata(&self) -> &SessionMetadata {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut SessionMetadata {
        &mut self.metadata
    }
    fn set_agent(&mut self, name: String) {
        self.set_agent_name(name);
    }
    fn attach_claim(&mut self, claim: &ClaimProvenance) {
        self.attach_claim_provenance(claim);
    }
}
