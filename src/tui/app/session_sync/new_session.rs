use std::path::Path;

use crate::session::Session;
use crate::tui::app::state::App;

mod persist;
mod state;

pub async fn create(app: &mut App, cwd: &Path, session: &mut Session) {
    let mut new_session = match Session::new().await {
        Ok(session) => session,
        Err(err) => return persist::fail_create(app, err),
    };
    if let Err(error) = session.save().await {
        persist::fail_save_current(app, error);
        return;
    }
    state::copy_preferences(app, session, &mut new_session);
    *session = new_session;
    persist::persist_new_session(app, session).await;
    state::reset_chat_state(app, session);
    super::refresh_sessions(app, cwd).await;
}
