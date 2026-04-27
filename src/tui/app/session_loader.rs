use crate::session::Session;
use crate::tui::app::resume_window::session_resume_window;
use anyhow::Result;

pub struct LoadedSession {
    pub session: Session,
    pub dropped: usize,
    pub file_bytes: u64,
}

pub async fn load_session_for_tui(id: &str) -> Result<LoadedSession> {
    match Session::load_tail(id, session_resume_window()).await {
        Ok(load) => Ok(LoadedSession {
            session: load.session,
            dropped: load.dropped,
            file_bytes: load.file_bytes,
        }),
        Err(native_error) => load_codex(id, native_error).await,
    }
}

async fn load_codex(id: &str, native_error: anyhow::Error) -> Result<LoadedSession> {
    match crate::session::load_or_import_session(id).await {
        Ok(session) => Ok(LoadedSession {
            session,
            dropped: 0,
            file_bytes: 0,
        }),
        Err(codex_error) => {
            Err(native_error.context(format!("Codex import also failed: {codex_error}")))
        }
    }
}
