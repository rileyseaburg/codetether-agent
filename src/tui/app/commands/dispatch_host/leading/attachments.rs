//! attachments command execution bindings.
use super::*;
pub(super) fn run(host: &mut Dispatch<'_>, kind: LeadingCommand, rest: &str) {
    let app = &mut *host.app;
    let cwd = host.cwd;
    match kind {
        codetether_slash_command::routing::LeadingCommand::Image => {
            let cleaned = rest.trim().trim_matches(|c| c == '"' || c == '\'');
            crate::tui::app::input::attach_image_command(
                app,
                cwd,
                cleaned,
                codetether_image::interactive::Presentation::Chat,
            );
            return;
        }
        codetether_slash_command::routing::LeadingCommand::File => {
            let cleaned = rest.trim().trim_matches(|c| c == '"' || c == '\'');
            crate::tui::app::file_share::command(app, cwd, cleaned);
            return;
        }
        codetether_slash_command::routing::LeadingCommand::Edit => {
            open_editor(
                app,
                cwd,
                rest.trim().trim_matches(|c| c == '"' || c == '\''),
            );
            return;
        }
        _ => unreachable!("command routed to the wrong native adapter"),
    }
}
