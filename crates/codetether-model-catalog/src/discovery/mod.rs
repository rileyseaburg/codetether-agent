//! Provider model discovery projection and bounded concurrent collection.
//!
//! CatalogRegistry supplies caller-owned provider lookup. collect_models
//! returns sorted discovery records; vscode projects them into editor records.
mod architecture;
pub mod collect;
pub mod convert;
pub mod ids;
pub mod options;
pub mod parameters;
pub mod pricing;
pub mod query;
mod top_provider;
pub mod types;
pub mod types_architecture;
pub mod types_pricing;
pub mod types_top_provider;
pub mod types_vscode;
pub mod vscode;
pub use collect::collect_models;
/// Caller-supplied source of configured provider names and implementations.
/// # Implementors
/// Native registries implement this to retain their own lookup and credentials.
pub trait CatalogRegistry: Sync {
    /// Return configured provider names in the registry's existing order.
    fn list(&self) -> Vec<&str>;
    /// Look up a configured provider without changing ownership.
    fn get(&self, name: &str) -> Option<std::sync::Arc<dyn codetether_provider_core::Provider>>;
}
