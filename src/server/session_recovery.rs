//! Native HTTP binding for shared identity-preserving session recovery.
mod authority;
mod errors;
mod handler;
mod host;
mod scope;
pub(super) use codetether_session_recovery::ResumeRequest;
pub(super) use errors::http_error;
pub(super) use handler::handle;
#[cfg(test)]
mod tests;
/// Recover exactly the requested native session identity.
pub(super) async fn resume(
    id: &str,
    request: ResumeRequest,
    workspace: &std::path::Path,
) -> anyhow::Result<serde_json::Value> {
    codetether_session_recovery::resume::<crate::session::Session>(id, request, workspace).await
}
