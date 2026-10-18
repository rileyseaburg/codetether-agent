//! Launch a configured approval reviewer and attach its eventual verdict.
//!
//! [start] checks runtime configuration and supported tools before obtaining
//! frontend context. Late verdicts are discarded by the existing approval queue.
use crate::{ReviewSubject, runtime::ReviewRuntime};
use codetether_approval_queue::ApprovalSnapshot;
use std::{future::Future, path::PathBuf};
/// Frontend context captured only for enabled, supported reviews.
/// # Examples
/// ```
/// let context = codetether_approval_review::dispatch::Context {
///     workspace: ".".into(), session_id: None, session_model: None };
/// assert!(context.session_id.is_none());
/// ```
pub struct Context {
    /// Directory the reviewer may inspect.
    pub workspace: PathBuf,
    /// Session whose goal supplies review context.
    pub session_id: Option<String>,
    /// Model fallback when review configuration does not name a model.
    pub session_model: Option<String>,
}
/// Installed runtime and native journal access used by dispatch.
/// # Implementors
/// Reuse the same reviewer host as direct reviewer execution.
pub trait Host: crate::Host + Send + Sync + 'static {
    /// Return the installed reviewer runtime only when enabled.
    fn enabled() -> Option<ReviewRuntime<Self::Registry>>;
    /// Read the current goal block for an optional session.
    fn goal(session_id: Option<&str>) -> impl Future<Output = Option<String>> + Send;
}
/// Start one asynchronous reviewer for a supported pending approval.
/// # Arguments
/// Receives the approval snapshot and a lazy frontend-context projection.
/// # Returns
/// Returns immediately after launching review, or without a task if disabled/unsupported.
/// # Examples
/// ```no_run
/// fn start<H: codetether_approval_review::dispatch::Host>(item: &codetether_approval_queue::ApprovalSnapshot)
/// where H::Registry: Send + 'static {
///     codetether_approval_review::dispatch::start::<H>(item, || codetether_approval_review::dispatch::Context {
///         workspace: ".".into(), session_id: None, session_model: None });
/// }
/// ```
pub fn start<H: Host>(item: &ApprovalSnapshot, context: impl FnOnce() -> Context)
where
    H::Registry: Send + 'static,
{
    let Some(review) = H::enabled() else {
        return;
    };
    if !codetether_proposed_content::supported(&item.tool) {
        return;
    }
    let Context {
        workspace,
        session_id,
        session_model,
    } = context();
    let id = item.id.clone();
    let mut subject = ReviewSubject {
        tool: item.tool.clone(),
        action: item.action.clone(),
        resource: item.resource.clone(),
        justification: item.justification.clone(),
        preview: item.preview.clone(),
        goal: None,
    };
    codetether_approval_queue::set_review(&id, None);
    tokio::spawn(async move {
        subject.goal = H::goal(session_id.as_deref()).await;
        let verdict = crate::review::<H>(
            &review.config,
            &review.registry,
            session_model.as_deref(),
            workspace,
            subject,
        )
        .await;
        tracing::info!(approval = %id, outcome = verdict.outcome.label(), "Reviewer verdict");
        codetether_approval_queue::set_review(&id, Some(verdict));
    });
}
