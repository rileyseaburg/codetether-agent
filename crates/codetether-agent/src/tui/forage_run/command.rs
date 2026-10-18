//! Application binding for the interactive forage command.
use super::host::Runtime;
use crate::{session::Session, tui::app::state::App};
/// Launch forage with the current session model.
pub fn handle_forage_command(app: &mut App, session: &Session, rest: &str) -> bool {
    codetether_forage_interactive::handle_forage_command::<Runtime>(
        app,
        session.metadata.model.clone(),
        rest,
    )
}
