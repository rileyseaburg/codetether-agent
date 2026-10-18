//! Peer identity, listener and discovery configuration.
//!
//! [`SpawnOptions`] contains startup inputs independent of the serving transport.
//! Use [`SpawnOptions::auto`] for defaults or the conversion supplied by a CLI.
mod convert;
mod defaults;
mod record;
pub use record::SpawnOptions;
