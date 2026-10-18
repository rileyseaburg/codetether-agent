//! Preserve durable identities and assign fresh transient continuation identities.
use super::Session;
/// Prepare a truncated session for interactive continuation.
/// # Arguments
/// * `session` - Loaded native session.
/// * `dropped` - Number of omitted messages.
/// # Returns
/// No original ID when untruncated; otherwise the source identity.
/// # Errors
/// Returns ownership-transfer errors before changing ID or title.
/// # Examples
/// ```no_run
/// # async fn example<S: codetether_session_resume::continuation::Session>(session: &mut S) {
/// let original = codetether_session_resume::continuation::fork_if_truncated(session, 3).await;
/// # }
/// ```
pub async fn fork_if_truncated<S: Session>(
    session: &mut S,
    dropped: usize,
) -> anyhow::Result<Option<String>> {
    if dropped == 0 {
        return Ok(None);
    }
    if session.is_persisted() {
        return Ok(Some(session.id().to_string()));
    }
    let original = session.id().to_string();
    let title = session
        .title()
        .map(str::to_owned)
        .unwrap_or_else(|| "large session".to_string());
    let continuation = uuid::Uuid::new_v4().to_string();
    S::reparent_owner(&original, &continuation).await?;
    session.set_id(continuation);
    session.set_title(format!("{title} (continued)"));
    Ok(Some(original))
}
