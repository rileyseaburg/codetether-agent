//! Cached provider model inventories for worker registration.
//!
//! Keep one [Catalog] for the worker process and call [Catalog::load] using
//! application registry [Host] bindings. Successful inventories are cached;
//! failed initialization can be retried on the next call.
mod catalog;
mod host;
mod load;
pub use catalog::Catalog;
pub use host::Host;

/// Provider names mapped to their advertised model descriptions.
pub type Models = std::collections::HashMap<String, Vec<codetether_provider_core::ModelInfo>>;
