//! The [`SessionBus`] handle itself — broadcast fan-out plus durable sink.

use std::sync::Arc;

use tokio::sync::{broadcast, mpsc};
use tracing::warn;

use super::sink::{NoopSink, SharedSink};
use crate::session::{DurableSink, SessionEvent};

/// Unified event bus for a single session.
///
/// See the [module docs](super) for the ephemeral-vs-durable model.
/// `SessionBus` is cheap to `clone`; every clone shares the same
/// broadcast channel, durable sink, and optional legacy mpsc forwarder.
///
/// # Examples
///
/// Fan out a single event to a broadcast subscriber and a durable sink:
///
/// ```rust,no_run
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// use std::sync::{Arc, Mutex};
/// use codetether_agent::session::{
///     DurableSink, SessionBus, SessionEvent, TokenDelta, TokenSource,
/// };
///
/// struct Mem(Arc<Mutex<usize>>);
/// impl DurableSink for Mem {
///     fn write(&self, _e: &SessionEvent) -> std::io::Result<()> {
///         *self.0.lock().unwrap() += 1;
///         Ok(())
///     }
/// }
///
/// let counter = Arc::new(Mutex::new(0));
/// let sink = Arc::new(Mem(counter.clone()));
/// let bus = SessionBus::new(64).with_durable_sink(sink);
/// let mut rx = bus.subscribe();
///
/// bus.emit(SessionEvent::TokenUsage(TokenDelta {
///     source: TokenSource::Root,
///     model: "m".into(),
///     prompt_tokens: 10, completion_tokens: 5, duration_ms: 0,
/// }));
///
/// let ev = rx.recv().await.unwrap();
/// assert!(ev.is_durable());
/// assert_eq!(*counter.lock().unwrap(), 1);
/// # });
/// ```
#[derive(Clone)]
pub struct SessionBus {
    tx: broadcast::Sender<SessionEvent>,
    sink: SharedSink,
    legacy: Option<mpsc::Sender<SessionEvent>>,
}

impl SessionBus {
    /// Construct a new bus with the given broadcast channel capacity.
    ///
    /// `capacity` controls how many pending events the broadcast channel
    /// retains per subscriber before emitting `RecvError::Lagged`. 64–256
    /// is typical for interactive TUIs.
    ///
    /// Starts with [`NoopSink`] and no legacy forwarder. Use
    /// [`Self::with_durable_sink`] and [`Self::with_legacy_mpsc`] to
    /// attach either.
    ///
    /// # Panics
    ///
    /// Panics if `capacity == 0` (same contract as `broadcast::channel`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::session::SessionBus;
    ///
    /// let bus = SessionBus::new(16);
    /// assert_eq!(bus.subscriber_count(), 0);
    /// ```
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        Self {
            tx,
            sink: Arc::new(NoopSink),
            legacy: None,
        }
    }

    /// Attach a durable write-ahead sink for `is_durable()` events.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::sync::Arc;
    /// use codetether_agent::session::{NoopSink, SessionBus};
    ///
    /// let bus = SessionBus::new(8).with_durable_sink(Arc::new(NoopSink));
    /// assert_eq!(bus.subscriber_count(), 0);
    /// ```
    pub fn with_durable_sink(mut self, sink: Arc<dyn DurableSink>) -> Self {
        self.sink = sink;
        self
    }

    /// Forward every emission to an existing legacy `mpsc::Sender` for
    /// backward compatibility with code paths that have not migrated.
    ///
    /// Emission is **non-blocking**: uses `try_send` and logs (does not
    /// propagate) failures. This preserves the lossy semantics the legacy
    /// channel already had.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// use codetether_agent::session::{SessionBus, SessionEvent};
    /// let (tx, mut rx) = tokio::sync::mpsc::channel(8);
    /// let bus = SessionBus::new(8).with_legacy_mpsc(tx);
    /// bus.emit(SessionEvent::Thinking);
    /// let ev = rx.recv().await.unwrap();
    /// assert!(matches!(ev, SessionEvent::Thinking));
    /// # });
    /// ```
    pub fn with_legacy_mpsc(mut self, tx: mpsc::Sender<SessionEvent>) -> Self {
        self.legacy = Some(tx);
        self
    }

    /// Subscribe to the ephemeral broadcast stream.
    ///
    /// All events — ephemeral and durable — are republished here for live
    /// UIs. Slow consumers will see `RecvError::Lagged(n)`; the durable
    /// sink is unaffected.
    pub fn subscribe(&self) -> broadcast::Receiver<SessionEvent> {
        self.tx.subscribe()
    }

    /// Current number of active broadcast subscribers.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::session::SessionBus;
    ///
    /// let bus = SessionBus::new(4);
    /// let _rx = bus.subscribe();
    /// assert_eq!(bus.subscriber_count(), 1);
    /// ```
    pub fn subscriber_count(&self) -> usize {
        self.tx.receiver_count()
    }

    /// Publish one event.
    ///
    /// For durable events the sink is written **first** (synchronously);
    /// on success the event is then broadcast. A sink failure is logged
    /// at `warn` and does not prevent the broadcast — observers still see
    /// the event even if durable persistence fell over.
    ///
    /// Sending on an empty broadcast channel is not an error; it simply
    /// means no live subscribers.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// use codetether_agent::session::{SessionBus, SessionEvent};
    /// let bus = SessionBus::new(4);
    /// bus.emit(SessionEvent::Done); // no subscribers → still ok
    /// # });
    /// ```
    pub fn emit(&self, event: SessionEvent) {
        if event.is_durable()
            && let Err(err) = self.sink.write(&event)
        {
            warn!(error = %err, "SessionBus: durable sink write failed");
        }

        if let Some(legacy) = self.legacy.as_ref()
            && let Err(err) = legacy.try_send(event.clone())
        {
            tracing::debug!(error = %err, "SessionBus: legacy mpsc forward failed");
        }

        let _ = self.tx.send(event);
    }
}
