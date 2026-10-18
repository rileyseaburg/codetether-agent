//! Native server startup services with application-owned HTTP routing.
//!
//! [`Host`] constructs cognition, installs tools and assembles request state for
//! the existing server startup sequence. [`Routes`] supplies application A2A and
//! session routing. `EMPTY_STATE` preserves the caller's explicit test policy.
use axum::Router;
use codetether_a2a_core::AgentCard;
use codetether_application_registry::{Application, ToolRegistry};
use codetether_bus_core::AgentBus;
use std::{marker::PhantomData, sync::Arc};
mod startup;

/// Native cognition runtime for the selected application tool registry.
#[allow(type_alias_bounds)]
pub type Runtime<T: Application, const EMPTY_STATE: bool> =
    codetether_native_cognition::CognitionRuntime<ToolRegistry<T>, EMPTY_STATE>;
/// Request state retaining the services initialized by server startup.
#[allow(type_alias_bounds)]
pub type State<T: Application, const EMPTY_STATE: bool> =
    codetether_server_state::AppState<Runtime<T, EMPTY_STATE>>;

/// Application routes surrounding native startup services.
/// # Implementors
/// Applications retain concrete A2A/session adapters and existing route composition.
pub trait Routes<T: Application, const EMPTY_STATE: bool> {
    /// Build the A2A card and transport routers with the shared agent bus.
    fn a2a(address: &str, bus: Arc<AgentBus>) -> (AgentCard, Router, Router);
    /// Compose application routes over the initialized native state.
    fn router(state: &State<T, EMPTY_STATE>, root: Router, nested: Router) -> Router;
}

/// Native startup host selecting a tool catalog, routes and initial-state policy.
/// # Examples
/// ```no_run
/// # async fn example<T: codetether_application_registry::Application, A: codetether_native_server::Routes<T, false>>() -> anyhow::Result<()> {
/// codetether_server_runtime::serve::<codetether_native_server::Host<T, A, false>>("127.0.0.1", 8000, "example").await
/// # }
/// ```
pub struct Host<T, A, const EMPTY_STATE: bool>(PhantomData<fn() -> (T, A)>);

mod router;
pub use router::build_router;
