//! Bind conversation resolution to application session persistence.
use super::*;
#[async_trait::async_trait]
impl codetether_a2a_http::conversation::ConversationStore for Store {
    type Session = Session;
    async fn create(&self) -> Result<Session> {
        Session::new().await
    }
    async fn resume(&self, id: &str) -> Result<Session> {
        Session::resume(id).await
    }
    fn exists(&self, id: &str) -> Result<bool> {
        Ok(Session::session_path(id)?.exists())
    }
    fn set_id(&self, session: &mut Session, id: &str) {
        session.id = id.to_string();
    }
}
