//! Project the validated native session header into a listing summary.
use crate::session::{Session, SessionSummary};
impl codetether_session_listing::indexed::Header for Session {
    fn into_summary(self, count: usize) -> SessionSummary {
        let directory = self.metadata.directory;
        SessionSummary {
            id: self.id,
            title: self.title,
            created_at: self.created_at,
            updated_at: self.updated_at,
            message_count: count,
            agent: self.agent,
            directory,
        }
    }
}
