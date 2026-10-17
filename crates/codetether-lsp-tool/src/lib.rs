//! LSP tool operations using a host-owned client identity and shared cache.
//!
//! [LspTool] manages workspace clients through [Host] and [State]. Private
//! modules parse actions, enforce capabilities, execute requests and format results.
mod actions;
mod args;
mod capability_gate;
mod construction;
mod dispatch;
mod execute;
mod format;
mod host;
mod manager;
mod manager_create;
mod manager_select;
mod operation;
mod schema;
mod schema_action;
mod shutdown;
mod state;
mod tool;
mod workspace;
pub use host::Host;
use operation::LspOperation;
pub use state::State;
/// Language-server tool retaining the host's shared workspace manager cache.
///
/// # Examples
/// ```rust
/// # fn example<H: codetether_lsp_tool::Host>() {
/// let _tool = codetether_lsp_tool::LspTool::<H>::new();
/// # }
/// ``
pub struct LspTool<H: Host> {
    pub(crate) root_uri: Option<String>,
    pub(crate) lsp_settings: Option<codetether_config::LspSettings>,
    host: std::marker::PhantomData<H>,
}
#[cfg(test)]
mod capability_gate_tests;
#[cfg(test)]
mod tests;
