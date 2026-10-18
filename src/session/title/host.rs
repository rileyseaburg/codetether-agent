//! Native title fields and modification timestamp for shared title operations.
use crate::{provider::Message, session::Session};
use codetether_session_title::TitleSession;
impl TitleSession for Session {
    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }
    fn messages(&self) -> &[Message] {
        &self.messages
    }
    fn store_title(&mut self, title: Option<String>) {
        self.title = title;
    }
    fn touch(&mut self) {
        self.updated_at = chrono::Utc::now();
    }
}
