//! Watchdog cancellation feedback, retry transfer and permanent failure.
//!
//! [apply] records a detected stall. [execute] retries only the actual in-flight
//! prompt after session return. [give_up] stops retries at the shared budget.
mod cancel;
mod give_up;
mod host;
mod retry;
mod state;
pub use cancel::apply;
pub use give_up::give_up;
pub use host::Host;
pub use retry::execute;
pub use state::{State, Ui};
