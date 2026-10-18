//! TUI startup order and concurrent loading without native session/UI ownership.
//!
//! Implement [Load], [Frame] and [Runtime] using native application bindings,
//! then call [run]. [Startup] retains independent startup results until session
//! policies and hydration consume them. The terminal and peer guards stay alive
//! through the event loop.
mod bus;
mod driver;
mod host;
mod project;
pub mod secrets;
mod startup;
mod state;
mod workspace;
pub use driver::run;
pub use host::{Frame, Host, Load, Runtime, Terminal};
pub use startup::load;
pub use state::{Resolved, Startup};

/// Initial and completed frontend state hydration.
pub mod hydrate;

/// Explicit-session scanning and startup session resolution.
pub mod resolve;

/// Startup trust/reviewer/session configuration ordering.
pub mod configuration;

/// Bounded session event and notice channel ownership.
pub mod channels;
/// Initial frontend construction and loading-frame rendering.
pub mod first_frame;
