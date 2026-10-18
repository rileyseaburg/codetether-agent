//! Interrupt and modal interception before ordinary shortcut selection.
//!
//! [handle] invokes [Host] handlers in established UI priority order.
mod host;
mod run;
pub use host::Host;
pub use run::handle;
