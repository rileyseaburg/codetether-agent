//! Runtime route-handler selection for browser pages.

use crate::browser_agent::network::js_route_handler;
use crate::browser_agent::page::BrowserPage;
use crate::browser_js::BrowserJsRouteHandler;

impl BrowserPage {
    pub(crate) fn active_route_handler(&self) -> Option<BrowserJsRouteHandler> {
        let fallback = js_route_handler(
            self.network_routes.clone(),
            self.session.url.clone(),
            self.security_policy.clone(),
        );
        Some(
            crate::browser_agent::context::service_worker::service_worker_route_handler(
                self.context_state.clone(),
                self.session.url.clone(),
                fallback,
            ),
        )
    }
}
