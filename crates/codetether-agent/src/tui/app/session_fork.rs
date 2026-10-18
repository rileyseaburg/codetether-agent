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

#[cfg(test)]
#[path = "session_loader_tests.rs"]
mod tests;
