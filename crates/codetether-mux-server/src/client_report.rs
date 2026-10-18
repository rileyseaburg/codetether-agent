//! Logging failed client task completion.

pub(super) fn report(result: Result<(), tokio::task::JoinError>) {
    if let Err(error) = result {
        tracing::warn!(%error, "Mux client task failed");
    }
}
