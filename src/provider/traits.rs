//! Provider contracts are shared with standalone adapters.

pub use codetether_provider_core::{ModelInfo, Provider};

#[cfg(test)]
#[path = "routing_identity.rs"]
mod routing_identity;
