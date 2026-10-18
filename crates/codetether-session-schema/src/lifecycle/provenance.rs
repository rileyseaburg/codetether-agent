//! provenance operations on the application-local shared session schema.
/// Internal implementation of the shared session lifecycle declaration.
#[doc(hidden)]
#[macro_export]
macro_rules! __session_lifecycle_provenance {
    ($name:ident) => {
        impl $name {
            /// Set the agent persona owning this session. Also updates the
            /// provenance record so audit logs reflect the new agent.
            pub fn set_agent_name(&mut self, agent_name: impl Into<::std::string::String>) {
                $crate::__private::metadata::lifecycle::set_agent_name(
                    &mut self.agent,
                    &mut self.metadata,
                    agent_name,
                );
            }

            /// Tag the session as having been dispatched by a specific A2A worker
            /// for a specific task.
            pub fn attach_worker_task_provenance(&mut self, worker_id: &str, task_id: &str) {
                $crate::__private::metadata::lifecycle::attach_worker_task(
                    &mut self.metadata,
                    worker_id,
                    task_id,
                );
            }

            /// Attach a claim-provenance record to the session's execution
            /// provenance.
            pub fn attach_claim_provenance(&mut self, claim: &$crate::__private::ClaimProvenance) {
                $crate::__private::metadata::lifecycle::attach_claim(&mut self.metadata, claim);
            }
        }
    };
}
