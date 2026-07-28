//! Network failure exception projection.

use crate::browser_agent::page::BrowserPage;
use crate::browser_session::NetworkEvent;

use super::exception_kind::RuntimeExceptionKind;
use super::exception_types::RuntimeException;

pub fn collect(page: &BrowserPage) -> Vec<RuntimeException> {
    page.network_events()
        .iter()
        .filter(|event| failed(event))
        .map(|event| RuntimeException {
            action: "network".into(),
            message: message(event),
            kind: RuntimeExceptionKind::Network,
        })
        .collect()
}

fn failed(event: &NetworkEvent) -> bool {
    event.status.is_some_and(|status| status >= 400)
        || event
            .route_result
            .as_ref()
            .is_some_and(|value| matches!(value.as_str(), "abort" | "blocked"))
}

fn message(event: &NetworkEvent) -> String {
    match (event.status, event.route_result.as_deref()) {
        (Some(status), Some(route)) => {
            format!("{} {} {} {}", event.method, event.url, status, route)
        }
        (Some(status), None) => format!("{} {} {}", event.method, event.url, status),
        (None, Some(route)) => format!("{} {} {}", event.method, event.url, route),
        (None, None) => format!("{} {}", event.method, event.url),
    }
}
