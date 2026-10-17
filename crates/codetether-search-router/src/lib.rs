//! Concrete search routing with caller-owned providers and memory embeddings.
//!
//! [SearchTool] exposes the shared search planner as a tool. [Registry] provides
//! configured provider access, while [run_router_search] also serves CLI callers.
//! Model resolution, backend dispatch, request schema and output formatting have
//! separate modules. Provider discovery and embedding selection stay with callers.
use codetether_memory_tool::AutoEmbedder;
use std::{marker::PhantomData, sync::Arc};
pub mod dispatch;
pub mod engine;
mod execute;
pub mod model;
mod registry;
mod schema;
mod tool;
pub use engine::run_router_search;
pub use registry::Registry;
/// Router tool backed by a shared registry and an automatic embedding factory.
///
/// # Examples
/// ```rust
/// # fn example<R:codetether_search_router::Registry>(registry:std::sync::Arc<R>) {
/// let _tool=codetether_search_router::SearchTool::<R,codetether_memory_tool::NoAutoEmbedder>::new(registry);
/// # }
/// ```
pub struct SearchTool<R, B> {
    registry: Arc<R>,
    factory: PhantomData<B>,
}
impl<R: Registry, B: AutoEmbedder> SearchTool<R, B> {
    /// Bind the tool to the existing provider registry.
    ///
    /// # Arguments
    /// * `registry` - Shared provider registry, retained without copying providers.
    /// # Returns
    /// A search tool using the chosen memory factory type.
    /// # Examples
    /// ```rust
    /// # fn example<R:codetether_search_router::Registry>(registry:std::sync::Arc<R>) {
    /// let _tool=codetether_search_router::SearchTool::<R,codetether_memory_tool::NoAutoEmbedder>::new(registry);
    /// # }
    /// ```
    pub fn new(registry: Arc<R>) -> Self {
        Self {
            registry,
            factory: PhantomData,
        }
    }
}
