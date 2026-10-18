//! Native execution of shared session-command classifications.
use crate::session::Session;
use crate::tui::app::state::App;
use codetether_slash_command::routing::{SessionCommand, session as classify};
use std::path::Path;

pub(super) async fn dispatch(
    app: &mut App,
    cwd: &Path,
    session: &mut Session,
    normalized: &str,
) -> bool {
    let Some((command, rest)) = classify(normalized) else {
        return false;
    };
    match command {
        SessionCommand::Forage => {
            crate::tui::forage_run::handle_forage_command(app, session, rest);
        }
        SessionCommand::Goal => {
            super::goal::handle(app, &session.id, rest).await;
        }
        SessionCommand::A2a => {
            super::a2a::handle(app, rest);
        }
        SessionCommand::Undo => {
            super::handle_undo_command(app, session, rest).await;
        }
        SessionCommand::Detach => {
            crate::tui::app::detach::handle_detach_command(app, session, rest).await;
        }
        SessionCommand::Fork => {
            super::handle_fork_command(app, cwd, session, rest).await;
        }
    }
    true
}
