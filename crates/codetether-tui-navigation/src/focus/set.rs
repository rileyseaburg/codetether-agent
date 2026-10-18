//! Agent focus set policy.
use super::Host;
use crate::Action;

pub(super) fn set_focus(app: &mut impl Host, next: Option<String>) {
    app.apply(Action::Status(next.as_ref().map_or_else(
        || "Focused main chat".to_string(),
        |name| format!("Focused agent: {name}"),
    )));
    super::swarm::sync(app, next.as_deref());
    app.set_active_agent(next);
    app.clear_message_cache();
    app.scroll_to_bottom();
}
