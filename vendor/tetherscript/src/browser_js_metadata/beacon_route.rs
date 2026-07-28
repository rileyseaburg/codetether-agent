//! Route-table bridge for beacon requests.

use super::super::super::{
    BrowserJsRouteAction, BrowserJsRouteRequest, SharedBrowserJsRouteHandler,
};

pub(super) fn run(
    url: &str,
    body: Option<String>,
    routes: &SharedBrowserJsRouteHandler,
) -> (Option<u16>, Option<&'static str>) {
    let Some(handler) = routes.borrow().clone() else {
        return (Some(204), None);
    };
    let action = handler.borrow_mut()(BrowserJsRouteRequest {
        method: "POST".into(),
        url: url.into(),
        headers: Vec::new(),
        body,
    });
    match action {
        BrowserJsRouteAction::Fulfill(response) => (Some(response.status), Some("fulfill")),
        BrowserJsRouteAction::Abort(_) => (None, Some("abort")),
        BrowserJsRouteAction::Blocked(_) => (None, Some("blocked")),
        BrowserJsRouteAction::Continue => (Some(204), Some("continue")),
        BrowserJsRouteAction::PassThrough => (Some(204), None),
    }
}
