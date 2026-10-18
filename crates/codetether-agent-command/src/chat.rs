//! Direct chat routing for managed child agents.

use crate::ChatHost;
use codetether_chat_model::{ChatMessage, MessageType};

/// Route ordinary text to an explicitly mentioned or focused owned child.
/// # Arguments
/// `app` provides session state and child operations; `input` is the submitted text.
/// # Returns
/// True only when a child message was dispatched.
/// # Examples
/// ```no_run
/// async fn route<H: codetether_agent_command::ChatHost>(host: &mut H) {
///     codetether_agent_command::chat::route(host, "@worker investigate").await;
/// }
/// ```
pub async fn route<H: ChatHost>(app: &mut H, input: &str) -> bool {
    if input.is_empty() || input.starts_with(['/', '!']) {
        return false;
    }
    let target = codetether_chat_submit::mention::parse_mention(input).or_else(|| {
        app.active()
            .map(str::to_string)
            .map(|name| (name, input.to_string()))
    });
    let Some((name, message)) = target else {
        return false;
    };
    let Some(parent) = app.parent() else {
        return false;
    };
    if !app.owned_by(&parent, &name) {
        return false;
    }
    send(app, &parent, &name, &message).await;
    true
}

/// Append a user message, dispatch it, present the result and clear input.
/// # Arguments
/// Receives the native host, explicit parent, child name and message text.
/// # Returns
/// Completes after updating focus and showing the dispatch result.
/// # Examples
/// ```no_run
/// async fn send<H: codetether_agent_command::ChatHost>(host: &mut H) {
///     codetether_agent_command::chat::send(host, "parent", "worker", "investigate").await;
/// }
/// ```
pub async fn send<H: ChatHost>(app: &mut H, parent: &str, name: &str, message: &str) {
    app.push(ChatMessage::new(
        MessageType::User,
        format!("To @{name}: {message}"),
    ));
    let result = app.message(parent, name, message).await;
    app.set_active(Some(name.to_string()));
    app.present(
        &result,
        format!("Message dispatched to @{name}"),
        &format!("Message to @{name} failed"),
    );
    app.clear_input();
}
