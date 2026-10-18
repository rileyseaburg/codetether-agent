//! Native recall access to application-owned session fields and loading.
use crate::session::Session;
impl codetether_native_recall::Session for Session {
    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }
    fn updated_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.updated_at
    }
    fn message_offset(&self) -> usize {
        self.message_offset()
    }
    fn message_count(&self) -> usize {
        self.message_count()
    }
    async fn load_tail(id: &str, limit: usize) -> anyhow::Result<Self> {
        Ok(Self::load_tail(id, limit).await?.session)
    }
}
