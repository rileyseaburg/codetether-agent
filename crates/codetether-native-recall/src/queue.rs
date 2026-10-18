//! Recall worker scheduling and removal.
/// Schedule recall indexing of a saved session.
/// # Arguments
/// * `session` - Session whose stable identity selects the worker.
/// # Returns
/// Immediately after scheduling asynchronous indexing.
/// # Examples
/// `schedule(&session)` preserves the normal shared worker lifecycle.
pub fn schedule<S: crate::Session>(session: &S) {
    codetether_recall_worker::schedule(
        codetether_recall_context::Source::id(session),
        std::sync::Arc::new(crate::Host::<S>::default()),
    );
}
/// Remove indexed recall evidence for a session.
/// # Arguments
/// * `id` - Identity of the deleted session.
/// # Returns
/// Unit after removing the shared worker's retained evidence.
/// # Errors
/// Propagates shared recall-worker removal errors.
/// # Examples
/// `remove::<S>(id).await?` uses the native session storage paths.
pub async fn remove<S: crate::Session>(id: &str) -> anyhow::Result<()> {
    codetether_recall_worker::remove(id, &crate::Host::<S>::default()).await
}
