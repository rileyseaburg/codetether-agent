use crate::tui::app::state::{App, approval_queue};
use crate::tui::chat::message::{ChatMessage, MessageType};

pub(super) fn decided(
    app: &mut App,
    id: &str,
    intent: super::intent::ApprovalIntent,
    text: String,
) {
    let live = crate::approval::live::decide(id, intent.live_decision());
    approval_queue::resolve(id);
    app.state.clear_input();
    push(app, text);
    app.state.status = if live {
        format!("Approval `{id}` sent to running tool")
    } else {
        format!("Approval `{id}` recorded")
    };
}

fn push(app: &mut App, text: String) {
    app.state
        .messages
        .push(ChatMessage::new(MessageType::System, text));
    app.state.scroll_to_bottom();
}
