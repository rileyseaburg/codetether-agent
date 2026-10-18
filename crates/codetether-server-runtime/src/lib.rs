//! HTTP server startup and service lifecycle orchestration.
//!
//! Implement [`Host`] for the application's native runtime adapters. [`serve`]
//! initializes services, launches the existing background tasks and binds HTTP.
mod cognition;
mod host;
mod parts;
mod prepare;
mod run;
mod services;
mod systems;
mod transport;
pub use host::Host;
pub use parts::Parts;
pub use run::serve;
