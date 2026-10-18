//! Event channel construction and transport attachment.
use crate::{SessionBus, SessionEvent, SharedSink};
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};
impl<S: Clone> SessionBus<S> {
    /// Construct a new bus with the given broadcast channel capacity.
    ///
    /// `capacity` controls how many pending events the broadcast channel
    /// retains per subscriber before emitting `RecvError::Lagged`. 64–256
    /// is typical for interactive TUIs.
    ///
    /// Starts with a no-op callback and no legacy forwarder. Use
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
    /// use codetether_session_bus::SessionBus;
    ///
    /// let bus = SessionBus::<()>::new(16);
    /// assert_eq!(bus.subscriber_count(), 0);
    /// ```
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        Self {
            tx,
            sink: Arc::new(|_| Ok(())),
            legacy: None,
        }
    }
    /// Attach a synchronous callback for durable events.
    /// # Examples
    /// ```
    /// let bus = codetether_session_bus::SessionBus::<()>::new(8)
    ///     .with_durable_sink(std::sync::Arc::new(|_| Ok(())));
    /// assert_eq!(bus.subscriber_count(), 0);
    /// ```
    pub fn with_durable_sink(mut self, sink: SharedSink<S>) -> Self {
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
    /// use codetether_session_bus::{SessionBus, SessionEvent};
    /// let (tx, mut rx) = tokio::sync::mpsc::channel(8);
    /// let bus = SessionBus::<()>::new(8).with_legacy_mpsc(tx);
    /// bus.emit(SessionEvent::Thinking);
    /// let ev = rx.recv().await.unwrap();
    /// assert!(matches!(ev, SessionEvent::Thinking));
    /// # });
    /// ```
    pub fn with_legacy_mpsc(mut self, tx: mpsc::Sender<SessionEvent<S>>) -> Self {
        self.legacy = Some(tx);
        self
    }
}
