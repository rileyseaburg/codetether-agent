//! Native bindings for the shared isolated computer-use worker.
pub use codetether_computer_worker::execute;

/// Serve isolated requests using direct native platform dispatch.
///
/// # Returns
/// Completes when the parent closes the protocol input.
/// # Examples
/// ```text
/// codetether windows computer-use-worker
/// ```
/// # Errors
/// Returns malformed-frame, serialization or stream I/O errors.
pub async fn serve() -> anyhow::Result<()> {
    codetether_computer_worker::serve_with(|request| async move {
        super::platform::dispatch(&request).await
    })
    .await
}
