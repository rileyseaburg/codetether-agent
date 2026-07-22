use crate::session::Session;

/// Fork a truncated session and transfer its durable child ownership.
pub(super) async fn fork_if_truncated(
    session: &mut Session,
    dropped: usize,
) -> anyhow::Result<Option<String>> {
    if dropped == 0 {
        return Ok(None);
    }
    let original = session.id.clone();
    let title = session
        .title
        .clone()
        .unwrap_or_else(|| "large session".to_string());
    let continuation = uuid::Uuid::new_v4().to_string();
    crate::tool::agent::persistence::reparent_owner(&original, &continuation).await?;
    session.id = continuation;
    session.title = Some(format!("{title} (continued)"));
    Ok(Some(original))
}

/// Fork for an interactive load and surface migration failures in app status.
pub(super) async fn fork_for_app(
    app: &mut super::state::App,
    session: &mut Session,
    dropped: usize,
) -> Option<Option<String>> {
    match fork_if_truncated(session, dropped).await {
        Ok(original) => Some(original),
        Err(error) => {
            app.state.status = format!("Failed to continue session: {error}");
            None
        }
    }
}

#[cfg(test)]
#[path = "session_loader_tests.rs"]
mod tests;
