//! Convert claim identity into execution provenance.
use super::TaskClaimResponse;
use codetether_provenance::ClaimProvenance;

impl TaskClaimResponse {
    pub fn into_provenance(self) -> ClaimProvenance {
        ClaimProvenance {
            worker_id: self.worker_id,
            task_id: self.task_id,
            run_id: self.run_id,
            attempt_id: self.attempt_id,
            tenant_id: self.tenant_id,
            agent_identity_id: self
                .agent_identity_id
                .or_else(|| self.user_id.map(|user_id| format!("user:{user_id}"))),
        }
    }
}
