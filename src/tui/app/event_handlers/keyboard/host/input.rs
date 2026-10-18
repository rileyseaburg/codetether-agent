use super::{Action, App};
pub(super) fn apply(app: &mut App, action: &Action) -> bool {
    match action {
        Action::SideQuestion => {
            super::super::super::side_question::compose_side_question(app);
        }
        Action::CopyReply => {
            super::super::super::copy_reply::handle_copy_reply(app);
        }
        Action::CopyTranscript => {
            super::super::super::copy_transcript::handle_copy_transcript(app);
        }
        Action::Voice => {
            super::super::handle_voice_key(app);
        }
        Action::Paste => {
            super::super::super::clipboard::handle_clipboard_paste(app);
        }
        Action::CancelWatchdog => {
            crate::tui::app::watchdog::handle_watchdog_cancel(&mut app.state);
        }
        _ => return false,
    }
    true
}
