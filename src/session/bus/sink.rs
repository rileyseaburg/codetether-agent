//! Durable sink trait and a no-op implementation for tests.
//!
//! Durable events (see [`SessionEvent::is_durable`]) are dispatched
//! synchronously to an implementor of [`DurableSink`]. The sink is
//! expected to buffer or persist the event before returning; the bus
//! guarantees no durable event is silently dropped.
//!
//! [`SessionEvent::is_durable`]: crate::session::SessionEvent::is_durable

use std::sync::Arc;

use crate::session::SessionEvent;

/// A write-ahead sink for durable [`SessionEvent`]s.
///
/// Implementors are responsible for persisting or queueing the event
/// before returning from [`DurableSink::write`]. Errors must be reported
/// through the returned `Result` so callers can fall back to logging —
/// **never swallow durable-write failures silently**.
///
/// # Thread-safety
///
/// Implementations must be `Send + Sync + 'static` and are shared behind
/// `Arc`. Interior mutability (e.g. `tokio::sync::Mutex`) is the
/// implementor's responsibility.
///
/// # Examples
///
/// A minimal in-memory sink useful in tests:
///
/// ```rust
/// use std::sync::{Arc, Mutex};
/// use codetether_agent::session::{DurableSink, SessionEvent};
///
/// struct Collector(Arc<Mutex<Vec<String>>>);
/// impl DurableSink for Collector {
///     fn write(&self, event: &SessionEvent) -> std::io::Result<()> {
///         self.0.lock().unwrap().push(format!("{event:?}"));
///         Ok(())
///     }
/// }
///
/// let c = Collector(Arc::new(Mutex::new(Vec::new())));
/// c.write(&SessionEvent::Done).unwrap();
/// ```
pub trait DurableSink: Send + Sync + 'static {
    /// Persist one durable event.
    ///
    /// # Errors
    ///
    /// Returns an `io::Error` if the underlying storage (file, network,
    /// object store) refused the write. Callers log and continue — a
    /// bus emit never panics on sink failure.
    fn write(&self, event: &SessionEvent) -> std::io::Result<()>;
}

/// A [`DurableSink`] that drops every event. Useful as a default when the
/// caller has no persistence requirement (e.g. in unit tests or one-shot
/// CLI invocations).
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::{DurableSink, NoopSink, SessionEvent};
///
/// let sink = NoopSink;
/// sink.write(&SessionEvent::Done).unwrap();
/// ```
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopSink;

impl DurableSink for NoopSink {
    fn write(&self, _event: &SessionEvent) -> std::io::Result<()> {
        Ok(())
    }
}

/// Type alias for the shared sink handle used inside [`crate::session::SessionBus`].
pub(crate) type SharedSink = Arc<dyn DurableSink>;
