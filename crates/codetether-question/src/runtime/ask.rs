//! Publish a validated batch and await a matching frontend response.
use super::{
    capability::SESSION,
    store::{Cleanup, Pending, lock},
};
use crate::{Answers, Draft, Error, Request};
use tokio::sync::oneshot;
/// Suspend the current turn for answers without blocking an executor thread.
///
/// # Arguments
/// * `request` — Validated batch to present.
/// * `session` — Optional injected session ID; must match the scoped turn.
/// # Returns
/// Ordered lists of user-selected labels/custom text.
/// # Errors
/// Returns [`Error::Unavailable`] outside a matching scope, [`Error::Busy`] if
/// a question is already open, or [`Error::Dismissed`] after dismissal.
/// # Examples
/// ```no_run
/// # async fn demo() -> Result<(), codetether_question::Error> {
/// use codetether_question::{Request, runtime};
/// let request = Request::parse(serde_json::json!({"question":"Continue?"}))?;
/// let answers = runtime::ask(request, Some("session-1")).await?;
/// assert_eq!(answers.len(), 1);
/// # Ok(()) }
/// ```
pub async fn ask(request: Request, session: Option<&str>) -> Result<Answers, Error> {
    let id = SESSION
        .try_with(Clone::clone)
        .map_err(|_| Error::Unavailable)?;
    if session.is_some_and(|s| s != id) {
        return Err(Error::Unavailable);
    }
    let (reply, receive) = oneshot::channel();
    {
        let mut pending = lock();
        if pending.contains_key(&id) {
            return Err(Error::Busy);
        }
        pending.insert(
            id.clone(),
            Pending {
                draft: Draft::new(request),
                reply: Some(reply),
            },
        );
    }
    let _cleanup = Cleanup(id);
    receive.await.unwrap_or(Err(Error::Dismissed))
}
