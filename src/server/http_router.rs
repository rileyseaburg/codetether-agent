//! Native runtime adapter routes supplied to the shared application router.
use super::{
    AppState, approval_routes, config_status, models_catalog, openai_http_host, session_realtime,
    session_routes, worker_modules,
};
use axum::Router;
use codetether_server_router::{Extensions, Transports};

impl codetether_server_router::HttpState for AppState {}

pub(super) fn build(state: &AppState, root: Router, nested: Router) -> Router {
    codetether_server_router::build(
        state.clone(),
        Extensions {
            approval: approval_routes::router(),
            sessions: session_routes::router(),
            realtime: codetether_session_realtime::http::router::<
                session_realtime::AppHost,
                AppState,
            >(),
            status: config_status::router(state.config.clone()),
            models: codetether_model_catalog::http::router::<models_catalog::AppHost, AppState>(),
            completions: codetether_openai_serving::http::router::<
                openai_http_host::AppHost,
                AppState,
            >(),
            workers: worker_modules::router(),
            resume: codetether_session_recovery::http::router::<crate::session::Session, AppState>(
            ),
        },
        Transports { root, nested },
        state.auth.clone(),
    )
}
