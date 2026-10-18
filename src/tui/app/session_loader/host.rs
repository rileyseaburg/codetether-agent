//! Native snapshot and archive-import operations for session loading.
use crate::session::Session;
use codetether_session_resume::loading::{Host, LoadedSession};
pub(super) struct Native;
impl Host for Native {
    type Session = Session;
    async fn load_tail(id: &str, window: usize) -> anyhow::Result<LoadedSession<Session>> {
        let load = Session::load_tail(id, window).await?;
        Ok(LoadedSession {
            session: load.session,
            dropped: load.dropped,
            file_bytes: load.file_bytes,
        })
    }
    async fn load_or_import(id: &str) -> anyhow::Result<Session> {
        crate::session::load_or_import_session(id).await
    }
}
