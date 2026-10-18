//! Coalescing proactive RLM workers over native storage adapters.
//!
//! [spawn] drains the latest snapshots from the shared runtime registry.
//! [Host] retains application session/cursor and sidecar access; this crate
//! owns summary execution, generation checks and projection commit order.
mod commit;
mod host;
mod incremental;
mod inherit;
mod legacy;
mod summarize;
mod worker;
pub use host::Host;
pub use worker::spawn;
