//! Session event transport with synchronous durability and lossy fan-out.
//!
//! [SessionBus] writes durable events through its [SharedSink] callback,
//! forwards to an optional legacy channel, then publishes to subscribers.
//! It is generic over the snapshot type in [SessionEvent].
mod bus;
mod construct;
mod emit;
mod sink;
mod subscribe;
pub use bus::SessionBus;
pub use codetether_session_events::SessionEvent;
pub use sink::SharedSink;
