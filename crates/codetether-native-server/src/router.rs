//! Native runtime adapter routes supplied to the shared application router.
use crate::State;
use axum::Router;
use codetether_application_registry::Application;
use codetether_approval_http as approval_routes;
use codetether_native_providers::ProviderRegistry;
use codetether_server_router::{Extensions, Transports};

/// Compose typed native HTTP routes under the shared mandatory middleware.
/// # Arguments
/// Receives initialized state, A2A transports and the executable's package identity.
/// # Returns
/// The fully bound application router with the existing route and layer order.
pub fn build_router<T, S, H, const EMPTY_STATE: bool>(
    state: &State<T, EMPTY_STATE>,
    root: Router,
    nested: Router,
    version: &'static str,
    name: &'static str,
) -> Router
where
    T: Application,
    S: codetether_session_http::GoalHost
        + codetether_session_fork::ForkSession
        + codetether_session_recovery::RecoverySession
        + 'static,
    H: codetether_session_realtime::Host<Session = S, Registry = ProviderRegistry>,
{
    codetether_server_router::build(
        state.clone(),
        Extensions {
            approval: approval_routes::router(),
            sessions: codetether_session_http::all_routes::<S, State<T, EMPTY_STATE>>(),
            realtime: codetether_session_realtime::http::router::<H, State<T, EMPTY_STATE>>(),
            status: codetether_server_status::router(state.inner.config.clone(), version, name),
            models: codetether_model_catalog::http::router::<ProviderRegistry, State<T, EMPTY_STATE>>(
            ),
            completions: codetether_openai_serving::http::router::<
                codetether_application_providers::OpenAiHttpHost<
                    codetether_native_providers::registry::application::Native,
                >,
                State<T, EMPTY_STATE>,
            >(),
            workers: codetether_server_router::worker_routes::<State<T, EMPTY_STATE>>(),
            resume: codetether_session_recovery::http::router::<S, State<T, EMPTY_STATE>>(),
        },
        Transports { root, nested },
        state.inner.auth.clone(),
    )
}
