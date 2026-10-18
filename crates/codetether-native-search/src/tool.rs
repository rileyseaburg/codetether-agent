//! Search router bound to the application's provider and embedding factories.
/// Search tool sharing the existing registry and memory backend selection.
///
/// # Examples
/// ```rust
/// let registry = std::sync::Arc::new(codetether_native_providers::ProviderRegistry::new());
/// let _tool = codetether_native_search::tool::SearchTool::new(registry);
/// ```
pub type SearchTool = codetether_search_router::SearchTool<
    codetether_native_providers::ProviderRegistry,
    codetether_native_memory::ApplicationAutoEmbedder,
>;
