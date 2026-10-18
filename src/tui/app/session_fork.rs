#[path = "session_fork_host.rs"]
mod host;
use crate::session::Session;
/// Prepare a bounded native session using shared continuation policy.
pub(super) async fn fork_if_truncated(
    session: &mut Session,
    dropped: usize,
) -> anyhow::Result<Option<String>> {
    codetether_session_resume::continuation::fork_if_truncated(session, dropped).await
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
