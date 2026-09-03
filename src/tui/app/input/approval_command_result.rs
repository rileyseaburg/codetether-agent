use crate::tui::app::state::{App, approval_queue};
use crate::tui::chat::message::{ChatMessage, MessageType};

pub(super) fn decided(
    app: &mut App,
    id: &str,
    intent: super::intent::ApprovalIntent,
    reason: &str,
    text: String,
) {
    let live = crate::approval::live::decide(id, intent.live_decision_with_reason(reason));
    approval_queue::resolve(id);
    app.state.approval_waiting = approval_queue::active().is_some();
    app.state.approval_preview_scroll = 0;
    app.state.clear_input();
    push(app, text);
    let mut status = if live {
        format!("{} `{id}`; decision sent to running tool", intent.label())
    } else {
        format!("{} `{id}`; decision recorded", intent.label())
    };
    if let Some(next) = approval_queue::active_id() {
        status.push_str(&format!("; next approval `{next}` pending"));
    }
    app.state.status = status;
}

fn push(app: &mut App, text: String) {
    app.state
        .messages
        .push(ChatMessage::new(MessageType::System, text));
    app.state.scroll_to_bottom();
}
