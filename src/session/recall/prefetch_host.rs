//! Native session view and existing prior-context authorization policy.
use crate::{provider::Message, session::Session};
use std::path::Path;
impl codetether_recall_context::Source for Session {
    fn prior_context_allowed(&self) -> bool {
        crate::session::helper::runtime::prior_context_allowed_for_session(self)
    }
    fn workspace(&self) -> Option<&Path> {
        self.metadata.directory.as_deref()
    }
    fn messages(&self) -> &[Message] {
        &self.messages
    }
    fn id(&self) -> &str {
        &self.id
    }
}
