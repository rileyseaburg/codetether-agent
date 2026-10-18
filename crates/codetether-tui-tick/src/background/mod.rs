//! Ordered background updates and completion-notice delivery.
//!
//! Implement [Host] for native queues, then call [drain] after an event-loop
//! iteration or [apply_notice] for a notice selected directly by the loop.
mod host;
mod run;
pub use host::Host;
pub use run::{apply_notice, drain};
