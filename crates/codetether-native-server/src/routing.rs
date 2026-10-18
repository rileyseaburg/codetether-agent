//! Native A2A and HTTP route composition for server startup.
use crate::{RealtimeHost, RealtimeSession, Routes, State};
use axum::Router;
use codetether_a2a_core::AgentCard;
use codetether_application_registry::Application;
use codetether_bus_core::AgentBus;
use std::{marker::PhantomData, sync::Arc};
/// Executable identity displayed by the native status routes.
/// # Implementors
/// Supply the embedding package's identity so native library versions never
/// replace the application's public name or version.
pub trait PackageIdentity {
    /// Application package version.
    fn version() -> &'static str;
    /// Application package name.
    fn name() -> &'static str;
}
/// Startup routes for catalog T, session S and executable identity I.
/// # Examples
/// This type is selected as the route adapter of the native startup Host.
/// ```
/// let _: Option<codetether_native_server::ApplicationRoutes<(), (), ()>> = None;
/// ```
pub struct ApplicationRoutes<T, S, I>(PhantomData<fn() -> (T, S, I)>);
impl<T, S, I, const EMPTY_STATE: bool> Routes<T, EMPTY_STATE> for ApplicationRoutes<T, S, I>
where
    T: Application,
    S: RealtimeSession
        + codetether_session_http::GoalHost
        + codetether_session_fork::ForkSession
        + codetether_session_recovery::RecoverySession,
    I: PackageIdentity,
{
    fn a2a(address: &str, bus: Arc<AgentBus>) -> (AgentCard, Router, Router) {
        codetether_native_a2a::server::A2AServer::<S>::startup_routers(address, bus)
    }
    fn router(state: &State<T, EMPTY_STATE>, root: Router, nested: Router) -> Router {
        crate::build_router::<T, S, RealtimeHost<S>, EMPTY_STATE>(
            state,
            root,
            nested,
            I::version(),
            I::name(),
        )
    }
}
