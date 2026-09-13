//! Slash commands that manage the session itself rather than the chat.
//!
//! `/goal`, `/forage`, `/a2a`, `/undo`, `/detach`, `/fork`. Returns `true`
//! when the command was one of these and has been handled.

use std::path::Path;

use crate::session::Session;
use crate::tui::app::state::App;
use crate::tui::app::text::command_with_optional_args;

pub(super) async fn dispatch(
    app: &mut App,
    cwd: &Path,
    session: &mut Session,
    normalized: &str,
) -> bool {
    if let Some(rest) = command_with_optional_args(normalized, "/forage") {
        crate::tui::forage_run::handle_forage_command(app, session, rest);
    } else if let Some(rest) = command_with_optional_args(normalized, "/goal") {
        super::goal::handle(app, session, rest).await;
    } else if let Some(rest) = command_with_optional_args(normalized, "/a2a") {
        super::a2a::handle(app, rest);
    } else if let Some(rest) = command_with_optional_args(normalized, "/undo") {
        super::handle_undo_command(app, session, rest).await;
    } else if let Some(rest) = command_with_optional_args(normalized, "/detach") {
        crate::tui::app::detach::handle_detach_command(app, session, rest).await;
    } else if let Some(rest) = command_with_optional_args(normalized, "/fork") {
        super::handle_fork_command(app, cwd, session, rest).await;
    } else {
        return false;
    }
    true
}
