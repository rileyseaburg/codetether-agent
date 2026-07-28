//! Bridge from page route tables to browser JavaScript routing.

use std::cell::RefCell;
use std::rc::Rc;

use crate::browser_js::{
    BrowserJsRouteAction, BrowserJsRouteFulfillment, BrowserJsRouteHandler, BrowserJsRouteRequest,
};

use crate::browser_agent::exports::SecurityPolicy;

use super::security_enforcement::{blocked_reason, metadata_for};
use super::{RouteAction, RouteRequest, SharedRouteTable};

pub(crate) fn js_route_handler(
    routes: SharedRouteTable,
    page_url: String,
    policy: SecurityPolicy,
) -> BrowserJsRouteHandler {
    Rc::new(RefCell::new(move |request: BrowserJsRouteRequest| {
        let route_request = RouteRequest::new(request.method, request.url)
            .with_headers(request.headers)
            .with_optional_body(request.body);
        let security = metadata_for(&page_url, &route_request, &policy);
        if let Some(reason) = blocked_reason(&security) {
            let action = RouteAction::abort(reason.clone());
            routes
                .borrow_mut()
                .push_log(route_request.with_security(security), action);
            return BrowserJsRouteAction::Blocked(reason);
        }
        if routes.borrow().routes().is_empty() {
            return BrowserJsRouteAction::PassThrough;
        }
        let action = routes
            .borrow_mut()
            .handle(route_request.with_security(security));
        match action {
            RouteAction::Continue => BrowserJsRouteAction::Continue,
            RouteAction::Abort(reason) => BrowserJsRouteAction::Abort(reason),
            RouteAction::Fulfill(response) => {
                BrowserJsRouteAction::Fulfill(BrowserJsRouteFulfillment {
                    status: response.status,
                    headers: response.headers,
                    body: response.body,
                })
            }
        }
    }))
}
