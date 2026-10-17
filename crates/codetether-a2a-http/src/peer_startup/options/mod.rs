//! Peer startup configuration and conversion boundary.
//!
//! [SpawnOptions] supplies listener and discovery settings; applications
//! implement conversion from their own command argument types.
mod convert;
mod defaults;
mod record;
pub use record::SpawnOptions;
