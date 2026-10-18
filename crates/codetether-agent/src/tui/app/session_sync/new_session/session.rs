//! Native session storage and metadata bindings for new-session creation.
use crate::session::{Session, SessionMetadata};
impl codetether_new_session::Session for Session {
    async fn new() -> anyhow::Result<Self> {
        Session::new().await
    }
    async fn save(&self) -> anyhow::Result<()> {
        Session::save(self).await
    }
    fn metadata(&self) -> &SessionMetadata {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut SessionMetadata {
        &mut self.metadata
    }
    fn id(&self) -> &str {
        &self.id
    }
    fn attach_global_bus_if_missing(&mut self) {
        Session::attach_global_bus_if_missing(self);
    }
}
