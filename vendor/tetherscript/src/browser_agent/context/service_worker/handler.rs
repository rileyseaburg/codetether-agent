//! JavaScript fetch route wrapper for active service workers.

use std::cell::RefCell;
use std::rc::Rc;

use crate::browser_agent::context::context_state::SharedContextState;
use crate::browser_js::{
    BrowserJsRouteAction, BrowserJsRouteFulfillment, BrowserJsRouteHandler, BrowserJsRouteRequest,
};

pub(crate) fn service_worker_route_handler(
    state: Option<SharedContextState>,
    page_url: String,
    fallback: BrowserJsRouteHandler,
) -> BrowserJsRouteHandler {
    Rc::new(RefCell::new(move |request: BrowserJsRouteRequest| {
        let fallback_action = fallback.borrow_mut()(request.clone());
        let can_intercept = matches!(
            fallback_action,
            BrowserJsRouteAction::PassThrough | BrowserJsRouteAction::Continue
        );
        if !can_intercept {
            return fallback_action;
        }
        let Some(state) = &state else {
            return BrowserJsRouteAction::PassThrough;
        };
        state
            .borrow_mut()
            .service_workers
            .intercept_fetch(&page_url, &request.url)
            .map(response_action)
            .unwrap_or(fallback_action)
    }))
}

fn response_action(response: super::CacheResponse) -> BrowserJsRouteAction {
    BrowserJsRouteAction::Fulfill(BrowserJsRouteFulfillment {
        status: response.status,
        headers: response.headers,
        body: response.body,
    })
}
