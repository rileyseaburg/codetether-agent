//! Applies claimed worker metadata without erasing durable runtime identity.

use super::{ClaimProvenance, ExecutionProvenance};

impl ExecutionProvenance {
    /// Merges task-claim fields into this execution provenance.
    ///
    /// A runtime identity already bound to the process takes precedence over
    /// the worker registry's claimed identity.
    ///
    /// # Examples
    ///
    /// ```
    /// use codetether_agent::provenance::{ClaimProvenance, ExecutionProvenance};
    /// let mut provenance = ExecutionProvenance::for_session("session", "author");
    /// let claim = ClaimProvenance {
    ///     worker_id: "worker-1".into(),
    ///     task_id: "task-1".into(),
    ///     ..ClaimProvenance::default()
    /// };
    /// provenance.apply_claim(&claim);
    /// assert_eq!(provenance.task_id.as_deref(), Some("task-1"));
    /// ```
    pub fn apply_claim(&mut self, claim: &ClaimProvenance) {
        self.apply_worker_task(&claim.worker_id, &claim.task_id);
        self.run_id = claim.run_id.clone().or_else(|| self.run_id.clone());
        self.attempt_id = claim.attempt_id.clone().or_else(|| self.attempt_id.clone());
        self.identity.tenant_id = claim
            .tenant_id
            .clone()
            .or_else(|| self.identity.tenant_id.clone());
        self.identity.agent_identity_id = self
            .identity
            .agent_identity_id
            .clone()
            .or_else(|| claim.agent_identity_id.clone());
    }
}
