//! Application choices required by shared catalog registration.
use crate::{Application, ToolRegistry};
use codetether_provider_core::Provider;
use codetether_tool_construction::{application::Factories, provider::ProviderTool};
use codetether_tool_core::Tool;
use std::sync::Arc;
/// Concrete tool constructors and feature choices for an application catalog.
/// # Implementors
/// Implement alongside the scalar factory groups; registration policy stays shared.
pub trait CatalogFactories: Factories + Default {
    /// Application whose registry receives the constructed tools.
    type Application: Application;
    /// Register the application's command-session tools.
    fn register_commands(registry: &mut ToolRegistry<Self::Application>);
    /// Whether scripting support is enabled in the embedding application.
    fn scripting_enabled() -> bool;
    /// Construct a provider-backed tool after shared provider selection.
    fn provider(kind: ProviderTool, provider: Arc<dyn Provider>, model: String) -> Arc<dyn Tool>;
}
