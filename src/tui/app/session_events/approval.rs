//! Approval request event rendering.

use crate::approval::LiveApprovalRequest;
use crate::tui::app::state::{App, approval_queue};
use crate::tui::chat::message::{ChatMessage, MessageType};

pub(super) fn request(app: &mut App, request: LiveApprovalRequest) {
    let pending = approval_queue::push(request);
    let count = approval_queue::len();
    let guidance = crate::tui::ui::trust_status::approval_guidance();
    let text = format!(
        "Approval pending ({count}): `{}` wants to {} `{}`. Ctrl+A approves, Ctrl+D denies. Slash: `/approve {}` or `/deny {}`. {guidance}",
        pending.tool, pending.action, pending.resource, pending.id, pending.id
    );
    app.state.status = text.clone();
    app.state
        .messages
        .push(ChatMessage::new(MessageType::System, text));
    app.state.scroll_to_bottom();
}
