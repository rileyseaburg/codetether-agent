use super::View;
use codetether_approval_queue as approval_queue;

pub fn decided(
    view: &mut impl View,
    id: &str,
    intent: codetether_approval_core::command::ApprovalIntent,
    reason: &str,
    text: String,
) {
    let live = codetether_approval_core::interaction::pending::decide(
        id,
        intent.live_decision_with_reason(reason),
    );
    approval_queue::resolve(id);
    view.set_waiting(approval_queue::active().is_some());
    view.reset_preview_scroll();
    view.clear_input();
    push(view, text);
    let mut status = if live {
        format!("{} `{id}`; decision sent to running tool", intent.label())
    } else {
        format!("{} `{id}`; decision recorded", intent.label())
    };
    if let Some(next) = approval_queue::active_id() {
        status.push_str(&format!("; next approval `{next}` pending"));
    }
    view.set_status(status);
}

fn push(view: &mut impl View, text: String) {
    view.push_system(text);
    view.scroll_to_bottom();
}
