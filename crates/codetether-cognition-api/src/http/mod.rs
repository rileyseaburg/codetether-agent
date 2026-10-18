//! Cognition and persona HTTP routes over caller-owned runtime operations.
//!
//! Implement [HttpState] and the runtime interfaces, then merge [router].
//! Interfaces separate lifecycle, persona, belief, workspace and proposal access.
mod beliefs_runtime;
mod lifecycle_runtime;
mod personas_runtime;
mod proposals_runtime;
mod workspace_runtime;
pub use beliefs_runtime::Beliefs;
pub use lifecycle_runtime::Lifecycle;
pub use personas_runtime::Personas;
pub use proposals_runtime::Proposals;
pub use workspace_runtime::Workspace;
mod beliefs;
mod control;
mod events;
mod governance;
mod persona_create;
mod persona_reap;
mod routes;
mod state;
mod workspace;
pub use routes::router;
pub use state::HttpState;

mod persona_routes;
mod runtime_routes;
mod state_routes;
