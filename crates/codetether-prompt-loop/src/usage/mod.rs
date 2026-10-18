//! Ordered provider metrics, durable goal usage and live usage reports.
//!
//! [record] updates shared provider metrics, accounts tokens and elapsed time,
//! then emits a usage event. [Host] supplies native goal persistence and state.
mod account;
mod host;
mod record;
pub use host::Host;
pub use record::record;
