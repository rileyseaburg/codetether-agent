//! Direct chat routing for managed child agents.

use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};

pub(crate) async fn route(app: &mut App, input: &str) -> bool {
    if input.is_empty() || input.starts_with(['/', '!']) {
        return false;
    }
    let target = crate::tui::app::input::mention_route::parse_mention(input).or_else(|| {
        app.state
            .active_spawned_agent
            .clone()
            .map(|name| (name, input.to_string()))
    });
    let Some((name, message)) = target else {
        return false;
    };
    let Some(parent) = app.state.session_id.clone() else {
        return false;
    };
    if !crate::tui::app::managed_agent::owned(&parent, &name) {
        return false;
    }
    send(app, &parent, &name, &message).await;
    true
}

pub(crate) async fn send(app: &mut App, parent: &str, name: &str, message: &str) {
    app.state.messages.push(ChatMessage::new(
        MessageType::User,
        format!("To @{name}: {message}"),
    ));
    let result = crate::tui::app::managed_agent::message(parent, name, message).await;
    app.state.active_spawned_agent = Some(name.to_string());
    super::ui::present(
        app,
        &result,
        format!("Message dispatched to @{name}"),
        &format!("Message to @{name} failed"),
    );
    app.state.clear_input();
}
