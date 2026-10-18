//! Native session operations for shared startup selection.
use crate::{
    bus::AgentBus,
    session::{Session, TailLoad},
};
use std::sync::Arc;
pub(super) struct Native;
impl codetether_tui_startup::resolve::Host for Native {
    type Session = Session;
    async fn load_tail(id: &str, window: usize) -> anyhow::Result<TailLoad> {
        Session::load_tail(id, window).await
    }
    async fn new_session() -> anyhow::Result<Session> {
        Session::new().await
    }
    fn label(session: &Session) -> String {
        session.display_label()
    }
    fn with_bus(session: Session, bus: Arc<AgentBus>) -> Session {
        session.with_bus(bus)
    }
    fn id(session: &Session) -> &str {
        &session.id
    }
    fn history_len(session: &Session) -> usize {
        session.history().len()
    }
    async fn fork_if_truncated(
        session: &mut Session,
        dropped: usize,
    ) -> anyhow::Result<Option<String>> {
        crate::tui::app::session_fork::fork_if_truncated(session, dropped).await
    }
}
