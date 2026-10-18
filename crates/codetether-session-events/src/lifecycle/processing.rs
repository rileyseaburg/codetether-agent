//! Worker notifications and startup timing bindings.
/// Operations used when entering or leaving processing.
/// # Implementors
/// Frontends forward worker notifications and expose existing timing guards.
#[allow(async_fn_in_trait)]
pub trait Processing {
    /// Notify the worker that processing has started.
    async fn processing_started(&mut self);
    /// Notify the worker that processing has stopped.
    async fn processing_stopped(&mut self);
    /// Whether request timing has already started.
    fn request_timing_started(&self) -> bool;
    /// Begin request timing.
    fn begin_request_timing(&mut self);
    /// Whether streaming has already started.
    fn streaming_started(&self) -> bool;
    /// Begin streaming presentation.
    fn begin_streaming(&mut self);
    /// Replace the frontend status.
    fn set_status(&mut self, status: String);
}
