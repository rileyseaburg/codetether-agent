//! Model API catalog, capabilities, and credential-aware lookups.
//!
//! [ModelCatalog] fetches catalog data and filters availability using the
//! shared secrets manager. Wire types retain the upstream JSON representation.
mod catalog;
mod wire;
pub use catalog::ModelCatalog;
pub use wire::*;
