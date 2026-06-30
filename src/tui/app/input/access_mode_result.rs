use crate::config::AccessMode;
use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};

use super::{parse, policy};

pub(super) async fn push(app: &mut App, cwd: &std::path::Path, mode: AccessMode) {
    let released = mode == AccessMode::Full && policy::release_active(app);
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

fn with_release_note(text: String, released: bool) -> String {
    if released {
        format!("{text}\nApproved the active paused tool request.")
    } else {
        text
    }
}
