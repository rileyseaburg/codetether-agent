//! Session, provider, and application-service bindings for shared setup.
use super::Native;
use std::{
    collections::HashSet,
    future::Future,
    path::{Path, PathBuf},
};
impl codetether_prompt_loop::setup::Host for Native<'_> {
    type Model = super::super::model::ModelState;
    type Router = crate::cognition::tool_router::ToolCallRouter;
    type Subagents = crate::session::helper::prompt_events::subagent_watch::SubAgentWatch;
    async fn thinking(&mut self) {
        if let Some(tx) = self.events {
            let _ = tx.send(crate::session::SessionEvent::Thinking).await;
        }
    }
    fn resolve_model(&mut self) -> anyhow::Result<Self::Model> {
        self.session.resolve_subcall_provider(self.registry);
        crate::session::helper::step_model_restore::resolve_initial(self.session, self.registry)
    }
    fn directory(&self) -> Option<PathBuf> {
        self.session.metadata.directory.clone()
    }
    fn capture_dirty(&self, cwd: &Path) -> impl Future<Output = HashSet<PathBuf>> + Send {
        crate::session::helper::validation::capture_git_dirty_files(cwd)
    }
    fn max_steps(&self) -> Option<usize> {
        self.session.max_steps
    }
    fn session_id(&self) -> &str {
        &self.session.id
    }
    fn router(&self) -> Option<Self::Router> {
        super::super::setup_support::router()
    }
}
