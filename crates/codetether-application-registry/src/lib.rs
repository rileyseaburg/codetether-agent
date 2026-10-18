//! Application tool collections, construction and runtime registry integrations.
//!
//! [ToolRegistry] owns a shared storage registry. [Application] supplies concrete
//! tool factories and runtime authorization, while the registry preserves batch
//! weak-reference initialization and collection access for its consumers.
use codetether_plugin_verification::PluginRegistry;
use codetether_provider_core::Provider;
use codetether_tool_core::Tool;
use std::{marker::PhantomData, sync::Arc};
mod application;
pub use application::Application;
mod access;
mod aliases;
mod batch;
pub mod catalog;
mod cognition;
pub mod commands;
mod construction;
mod installation;
mod lifecycle;
mod search;
pub mod session_tools;
mod swarm;
mod workspace;
pub mod workspace_factory;

/// Tool registry whose factories and policy belong to an embedding application.
/// # Examples
/// ```no_run
/// # fn example<H: codetether_application_registry::Application>() {
/// let registry = codetether_application_registry::ToolRegistry::<H>::new();
/// assert!(registry.is_empty());
/// # }
/// ```
pub struct ToolRegistry<H: Application> {
    inner: codetether_tool_registry::ToolRegistry,
    _application: PhantomData<fn() -> H>,
}
impl<H: Application> std::fmt::Debug for ToolRegistry<H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.inner, f)
    }
}

/// Proactive context supplied by application-registered diagnostic tools.
pub mod proactive;

#[cfg(test)]
mod command_guard_tests;
#[cfg(test)]
mod temp_write_guard_tests;
