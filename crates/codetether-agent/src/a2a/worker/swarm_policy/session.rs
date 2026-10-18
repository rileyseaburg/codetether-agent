//! Application permission and title services for native swarm execution.
impl codetether_native_worker::swarm_policy::Session for crate::session::Session {
    fn swarm_prior_context_allowed(&self) -> bool {
        crate::session::helper::runtime::prior_context_allowed_for_session(self)
    }
    async fn generate_swarm_title(&mut self) -> anyhow::Result<()> {
        self.generate_title().await
    }
}
