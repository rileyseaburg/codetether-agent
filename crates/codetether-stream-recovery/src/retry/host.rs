use crate::StreamRecoveryState;
use std::time::{Duration, Instant};

/// Borrowed state changed while admitting and submitting a reconnect.
///
/// # Examples
/// ```text
/// State { recovery, processing, inflight, last_event }
/// ```
pub struct State<'a> {
    /// Pending prompt and reconnect attempt count.
    pub recovery: &'a mut StreamRecoveryState,
    /// Whether the UI is processing a request.
    pub processing: &'a mut bool,
    /// Prompt associated with the in-flight request.
    pub inflight: &'a mut Option<String>,
    /// Last observed activity for watchdog timing.
    pub last_event: &'a mut Option<Instant>,
}

/// Native session ownership and presentation operations for reconnects.
/// # Implementors
/// The TUI adapter supplies runtime requests and asynchronous sleeping.
pub trait Host {
    /// Owned session admitted for the retry.
    type Session;
    /// Request returned intact when submission fails.
    type Request;
    /// Shared provider registry supplied by the caller.
    type Provider;
    /// Borrow retry and processing state.
    fn state(&mut self) -> State<'_>;
    /// Take session ownership if it is available for prompting.
    fn take_session(&mut self) -> Option<Self::Session>;
    /// Publish the reconnect attempt and scroll the UI.
    fn banner(&mut self);
    /// Wait for the selected backoff duration.
    fn wait(&self, duration: Duration) -> impl Future<Output = ()>;
    /// Start request timing after backoff completes.
    fn begin_timing(&mut self);
    /// Build a text-only request, retaining the session's directory.
    fn request(session: Self::Session, prompt: String, provider: &Self::Provider) -> Self::Request;
    /// Submit ownership, returning the request if the runtime is unavailable.
    fn submit(&self, request: Self::Request) -> impl Future<Output = Result<(), Self::Request>>;
    /// Restore the session from an unaccepted request.
    fn restore(&mut self, request: Self::Request);
}
