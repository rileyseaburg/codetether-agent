//! Native HTTP binding for shared identity-preserving session recovery.
mod host;
#[cfg(test)]
pub(super) use codetether_session_access::http_error;
#[cfg(test)]
pub(super) use codetether_session_recovery::ResumeRequest;
#[cfg(test)]
mod tests;
/// Recover exactly the requested native session identity.
#[cfg(test)]
pub(super) async fn resume(
    id: &str,
    request: ResumeRequest,
    workspace: &std::path::Path,
) -> anyhow::Result<serde_json::Value> {
    codetether_session_recovery::resume::<crate::session::Session>(id, request, workspace).await
}
