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
