use crate::config::AccessMode;
use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};

use super::{parse, policy};

pub(super) async fn push(app: &mut App, cwd: &std::path::Path, mode: AccessMode) {
    let released = (mode == AccessMode::Full)
        .then(|| crate::tui::app::input::approval_command::release_all(app));
    let text = format!(
        "Access mode set to `{}`.\n{}",
        parse::label(mode),
        policy::summary(cwd).await
    );
    app.state.messages.push(ChatMessage::new(
        MessageType::System,
        with_release_note(text, released),
    ));
    app.state.status = format!("Access mode: {}", parse::label(mode));
    app.state.scroll_to_bottom();
    app.state.clear_input();
}

fn with_release_note(text: String, released: Option<Result<usize, String>>) -> String {
    match released {
        Some(Ok(count)) if count > 0 => {
            format!("{text}\nApproved {count} paused tool request(s).")
        }
        Some(Err(error)) => format!("{text}\nCould not release paused requests: {error}"),
        Some(Ok(_)) | None => text,
    }
}
