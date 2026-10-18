//! MCP serving over application-bound providers, tools and transport.
//!
//! [`ApplicationHost`] implements both serving host contracts when its type
//! arguments are application provider/tool registries and their MCP server.
//! It retains provider selection and stdio/bus setup for [`super::serve`].
use std::marker::PhantomData;
mod providers;
mod server;

/// Bind the provider registry, tool registry and MCP server used by stdio serving.
/// # Examples
/// ```no_run
/// # fn example<P, T, S>() where P: codetether_application_providers::Application,
/// # T: codetether_application_registry::Application,
/// # S: codetether_mcp_server::Application<Registry = codetether_application_registry::ToolRegistry<T>> {
/// type Bound<P, T, S> = codetether_mcp_cli::serve::application::ApplicationHost<
///     codetether_application_providers::ProviderRegistry<P>,
///     codetether_application_registry::ToolRegistry<T>,
///     codetether_mcp_server::ApplicationServer<S>>;
/// fn requires_host<H: codetether_mcp_cli::serve::Host>() {}
/// requires_host::<Bound<P, T, S>>();
/// # }
/// ```
pub struct ApplicationHost<P, T, S>(PhantomData<fn() -> (P, T, S)>);
