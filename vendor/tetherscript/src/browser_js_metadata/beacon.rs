//! Deterministic `navigator.sendBeacon` network logging.

use std::collections::HashMap;

use crate::js::JsValue;

use super::super::{native, record_network_event, SharedBrowserJsRouteHandler};

#[path = "beacon_route.rs"]
mod beacon_route;

pub(super) fn install(
    navigator: &mut HashMap<String, JsValue>,
    routes: SharedBrowserJsRouteHandler,
) {
    navigator.insert(
        "sendBeacon".into(),
        native("navigator.sendBeacon", None, move |args| {
            let url = args.first().unwrap_or(&JsValue::Undefined).display();
            let body = args.get(1).map(JsValue::display);
            let (status, route) = beacon_route::run(&url, body.clone(), &routes);
            record_network_event("POST", &url, status, Some(detail(route, body.as_deref())));
            Ok(JsValue::Bool(true))
        }),
    );
}

fn detail(route: Option<&str>, body: Option<&str>) -> String {
    let base = format!("beacon:body_bytes={}", body.map(str::len).unwrap_or(0));
    match route {
        Some(route) => format!("{base}:{route}"),
        None => base,
    }
}
