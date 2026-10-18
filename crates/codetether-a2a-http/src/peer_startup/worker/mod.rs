//! Optional worker-local A2A peer configuration and startup.
//!
//! [maybe_start_worker_a2a_peer] reads worker discovery settings, builds shared
//! startup options and invokes the supplied native endpoint callback. Disabled
//! or failed peers leave the existing worker transport available.
mod environment;
mod run;
mod url;
pub use run::maybe_start_worker_a2a_peer;
