//! Native fields used by interactive trailing-turn forks.
use crate::provider::Message;
use crate::session::{Session, SessionMetadata, pages::PageKind};
impl codetether_session_fork::interactive::Session for Session {
    fn pages(&self) -> &[PageKind] {
        &self.pages
    }
    fn install_history(&mut self, messages: Vec<Message>, pages: Vec<PageKind>) {
        self.messages = messages.into();
        self.pages = pages.into();
    }
    fn metadata_mut(&mut self) -> &mut SessionMetadata {
        &mut self.metadata
    }
    fn attach_global_bus_if_missing(&mut self) {
        Session::attach_global_bus_if_missing(self);
    }
}
