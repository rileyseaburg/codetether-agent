//! Apply execution identifiers and worker ownership.

use super::{ExecutionOrigin, ExecutionProvenance};
impl ExecutionProvenance {
    ///
    /// # Examples
    /// ```
    /// use codetether_provenance::{ExecutionProvenance, ExecutionOrigin};
    /// let mut record = ExecutionProvenance::for_operation("worker", ExecutionOrigin::Worker);
    /// record.set_agent_name("reviewer");
    /// assert_eq!(record.identity.agent_name,"reviewer");
    /// ```
    pub fn set_agent_name(&mut self, agent_name: &str) {
        self.identity.agent_name = agent_name.to_string();
    }

    ///
    /// # Examples
    /// ```
    /// use codetether_provenance::{ExecutionProvenance, ExecutionOrigin};
    /// let mut record = ExecutionProvenance::for_operation("worker", ExecutionOrigin::Worker);
    /// record.apply_worker_task("worker-1","task-1");
    /// assert_eq!(record.task_id.as_deref(),Some("task-1"));
    /// ```
    pub fn apply_worker_task(&mut self, worker_id: &str, task_id: &str) {
        self.identity.origin = ExecutionOrigin::Worker;
        self.identity.worker_id = Some(worker_id.to_string());
        self.task_id = Some(task_id.to_string());
    }

    ///
    /// # Examples
    /// ```
    /// use codetether_provenance::{ExecutionProvenance, ExecutionOrigin};
    /// let mut record = ExecutionProvenance::for_operation("worker", ExecutionOrigin::Worker);
    /// record.set_run_id("run");
    /// assert_eq!(record.run_id.as_deref(),Some("run"));
    /// ```
    pub fn set_run_id(&mut self, run_id: impl Into<String>) {
        self.run_id = Some(run_id.into());
    }
}
