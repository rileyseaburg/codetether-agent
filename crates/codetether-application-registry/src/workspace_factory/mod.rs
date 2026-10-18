//! Workspace registry construction using the application's existing lifecycle.
//!
//! [`WorkspaceFactory`] combines application registry factories with rooted LSP
//! and optional scripting tools supplied by a [`codetether_workspace_tools::Host`].
//! Pass the bound type to the functions in `codetether_workspace_tools::composition`.
use std::marker::PhantomData;
mod factory;
mod host;
mod session;

/// Static workspace factory binding registry ownership to rooted tool creation.
///
/// # Examples
/// ```no_run
/// # fn example<A, W>() where A: codetether_application_registry::Application,
/// # W: codetether_workspace_tools::Host<Registry = codetether_application_registry::ToolRegistry<A>> {
/// type Factory<A, W> = codetether_application_registry::workspace_factory::WorkspaceFactory<A, W>;
/// let registry = codetether_workspace_tools::composition::with_defaults::<Factory<A, W>>(
///     std::path::Path::new("."), codetether_workspace_tools::Capability::ReadOnly);
/// # }
/// ```
pub struct WorkspaceFactory<A, W>(PhantomData<fn() -> (A, W)>);
