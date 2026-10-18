//! Native registry and workspace constructors for worker tool assembly.
//!
//! [`Native`] supplies the existing application registry plus native LSP,
//! multi-edit and undo tools. Its workflow parameter supplies MCP, RLM, Ralph
//! and Go construction through the shared workflow contract.
use std::marker::PhantomData;
mod registry;
mod workflow;
mod workspace;
/// Worker tool host with registry application `A` and workflow constructors `W`.
///
/// This marker is selected as a type by worker tool assembly.
///
/// # Examples
/// ```no_run
/// # fn example<A: codetether_application_registry::Application, W: codetether_worker_tools::Workflow>() {
/// let _registry = <codetether_native_worker::tool_registry::Native<A, W> as codetether_worker_tools::Registry>::new_registry();
/// # }
/// ```
pub struct Native<A, W>(PhantomData<fn() -> (A, W)>);
