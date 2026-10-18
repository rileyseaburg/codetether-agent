//! Managed child session initialization and live-parent resume overlays.
//!
//! Implement [SessionAccess] and [ChildSession] for an existing session type.
//! [create] applies spawn defaults and a workspace-aware system message.
//! [parent_prior_context_allowed] resolves the inherited access ceiling;
//! [apply_overlay] refreshes model, access policy and unpinned workspaces.
//! Persistence, tracked-message mutation and bus ownership remain host operations.
mod access;
mod create;
mod overlay;
mod parent_policy;
mod session;
mod settings;
pub use access::SessionAccess;
pub use create::create;
pub use overlay::apply as apply_overlay;
pub use parent_policy::resolve as parent_prior_context_allowed;
pub use session::ChildSession;
pub use settings::Settings;

mod isolated;
mod workspace_binding;
pub use isolated::create_isolated;

mod message;
pub use message::prepare_message;

/// Implementation support for application-local session bindings.
#[doc(hidden)]
pub mod session_binding;

mod factory_binding;

/// Dependencies used by application-local factory declarations.
#[doc(hidden)]
pub mod __private {
    pub use anyhow::Result;
    pub use codetether_agent_input::{Params, SpawnRequest};
    pub use codetether_child_workspace::Handoff;
}
