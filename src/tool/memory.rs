//! Application memory aliases and automatic embedding-provider selection.
pub use codetether_memory_tool::{MemoryEntry, MemoryStats, fusion, query_match, search, semantic};
/// Store using the application's existing automatic embedding selection.
pub type MemoryStore = codetether_memory_tool::Store<ApplicationAutoEmbedder>;
/// Memory tool using the application's existing provider selection.
pub type MemoryTool = codetether_memory_tool::Runtime<ApplicationAutoEmbedder>;
/// Factory preserving the application-owned provider registry and embedding policy.
///
/// # Examples
/// ```rust
/// let _factory = codetether_agent::tool::memory::ApplicationAutoEmbedder;
/// ```
pub type ApplicationAutoEmbedder =
    codetether_application_providers::embedding::AutoEmbedder<crate::provider::ProviderRegistry>;
/// Stateless factory value retained for the native construction syntax.
#[allow(non_upper_case_globals)]
pub const ApplicationAutoEmbedder: ApplicationAutoEmbedder = ApplicationAutoEmbedder::new();
