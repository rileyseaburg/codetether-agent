//! Native storage identity and child-owner bindings for continuation.
use crate::session::Session;
impl codetether_session_resume::continuation::Session for Session {
    fn is_persisted(&self) -> bool {
        self.storage.is_persisted()
    }
    fn id(&self) -> &str {
        &self.id
    }
    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }
    async fn reparent_owner(original: &str, continuation: &str) -> anyhow::Result<()> {
        crate::tool::agent::persistence::reparent_owner(original, continuation)
            .await
            .map(|_| ())
    }
    fn set_id(&mut self, id: String) {
        self.id = id;
    }
    fn set_title(&mut self, title: String) {
        self.title = Some(title);
    }
}
