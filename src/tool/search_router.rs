//! Search router bound to the application's provider and embedding factories.
/// Search tool sharing the existing registry and memory backend selection.
///
/// # Examples
/// ```rust
/// let registry = std::sync::Arc::new(codetether_agent::provider::ProviderRegistry::new());
/// let _tool = codetether_agent::tool::search_router::SearchTool::new(registry);
/// ```
pub type SearchTool = codetether_search_router::SearchTool<
    crate::provider::ProviderRegistry,
    super::memory::ApplicationAutoEmbedder,
>;
