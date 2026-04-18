//! Unified session event bus: ephemeral broadcast + durable write-ahead.
//!
//! The [`SessionBus`] is the single emission point for everything the
//! session engine wants to tell the outside world — streaming text, tool
//! activity, token estimates, RLM progress, compaction lifecycle. It
//! replaces the ad-hoc `mpsc::Sender<SessionEvent>` plumbing that
//! previously had to be re-wired for every new observability surface
//! (status line, `/tokens` command, RLM view, JSONL flywheel).
//!
//! # Two transports, one enum
//!
//! A single [`SessionEvent`] instance flows through two independent
//! transports depending on [`SessionEvent::is_durable`]:
//!
//! 1. **Ephemeral broadcast** — `tokio::sync::broadcast`, lossy under
//!    load. Subscribers (TUI, progress spinners) get best-effort delivery
//!    and `RecvError::Lagged` if they fall behind. This is acceptable
//!    for observability signals: the next tick supersedes a dropped one.
//! 2. **Durable write-ahead sink** — an `Arc<dyn `[`DurableSink`]`>`.
//!    Durable events (`TokenUsage`, `RlmComplete`, `Compaction*`,
//!    `ContextTruncated`) are handed synchronously to the sink before the
//!    emit call returns. The JSONL → MinIO flywheel subscribes here so
//!    training-data completeness does not depend on broadcast backpressure.
//!
//! All events — ephemeral and durable — are *also* published on the
//! broadcast channel so live UIs can render both without subscribing
//! twice.
//!
//! # Backward compatibility
//!
//! Existing code paths that hold a `tokio::sync::mpsc::Sender<SessionEvent>`
//! continue to work: the bus can be constructed with an optional legacy
//! forwarder via [`SessionBus::with_legacy_mpsc`]. Call sites that have
//! not yet migrated see no change.
//!
//! # Quick start
//!
//! ```rust,no_run
//! # tokio::runtime::Runtime::new().unwrap().block_on(async {
//! use codetether_agent::session::{SessionBus, SessionEvent};
//!
//! let bus = SessionBus::new(128);
//! let mut rx = bus.subscribe();
//!
//! bus.emit(SessionEvent::Thinking);
//!
//! let ev = rx.recv().await.unwrap();
//! assert!(matches!(ev, SessionEvent::Thinking));
//! # });
//! ```

mod handle;
mod sink;

pub use self::handle::SessionBus;
pub use self::sink::{DurableSink, NoopSink};
