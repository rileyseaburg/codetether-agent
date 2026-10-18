//! Native history, timestamp and persistence bindings for undo.
use crate::provider::Message;
use crate::session::{Session, pages::PageKind};
use chrono::{DateTime, Utc};
impl codetether_turn_undo::command::Session for Session {
    fn history(&mut self) -> (&mut Vec<Message>, &mut Vec<PageKind>) {
        (&mut self.messages, &mut self.pages)
    }
    fn set_updated_at(&mut self, timestamp: DateTime<Utc>) {
        self.updated_at = timestamp;
    }
    async fn save(&self) -> anyhow::Result<()> {
        Session::save(self).await
    }
}
