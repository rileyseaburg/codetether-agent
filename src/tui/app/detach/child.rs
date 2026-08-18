//! Build a persisted point-in-time copy of a session for `/detach`.

use crate::session::Session;

/// Build and persist a child session copying `parent`'s thread state.
pub(super) async fn build_child(parent: &Session) -> anyhow::Result<Session> {
    let mut child = Session::new().await?;
    child.messages = tail(
        parent,
        crate::tui::app::resume_window::session_resume_window(),
    );
    child.pages = crate::session::pages::classify_all(&child.messages);
    child.metadata = parent.metadata.clone();
    child.agent = "detached".to_string();
    child.title = parent
        .title
        .as_ref()
        .map(|t| format!("{t} (detached)"))
        .or_else(|| Some("detached".to_string()));
    child.attach_global_bus_if_missing();
    child.save().await?;
    Ok(child)
}

fn tail(parent: &Session, window: usize) -> Vec<crate::provider::Message> {
    let start = parent.messages.len().saturating_sub(window);
    parent.messages[start..].to_vec()
}

#[cfg(test)]
#[path = "child_tests.rs"]
mod tests;
