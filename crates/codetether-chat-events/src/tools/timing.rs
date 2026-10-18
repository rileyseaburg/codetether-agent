//! Worker notifications and tool-start timing operations.
/// Timing operations required before a tool begins.
/// # Implementors
/// Frontends forward their worker notification and existing timing methods.
#[allow(async_fn_in_trait)]
pub trait Timing {
    /// Notify the worker that processing is active.
    async fn processing_started(&mut self);
    /// Whether request timing already exists.
    fn request_timing_started(&self) -> bool;
    /// Start request timing.
    fn begin_request_timing(&mut self);
    /// Whether streaming timing already exists.
    fn streaming_started(&self) -> bool;
    /// Start streaming timing.
    fn begin_streaming(&mut self);
}
