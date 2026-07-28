//! HAR-style response projection.

use crate::browser_agent::network::RouteAction;
use crate::browser_session::NetworkEvent;

use super::har_types::BrowserHarResponse;

pub fn response(event: &NetworkEvent, action: Option<&RouteAction>) -> BrowserHarResponse {
    match action {
        Some(RouteAction::Abort(reason)) => failed(event, reason),
        Some(RouteAction::Fulfill(reply)) => BrowserHarResponse {
            status: reply.status,
            status_text: status_text(reply.status).into(),
            headers: reply.headers.clone(),
            content_text: Some(reply.body.clone()),
            route_result: Some("fulfill".into()),
        },
        Some(RouteAction::Continue) | None => observed(event),
    }
}

fn observed(event: &NetworkEvent) -> BrowserHarResponse {
    let status = event.status.unwrap_or(0);
    BrowserHarResponse {
        status,
        status_text: status_text(status).into(),
        headers: Vec::new(),
        content_text: None,
        route_result: event.route_result.clone(),
    }
}

fn failed(event: &NetworkEvent, reason: &str) -> BrowserHarResponse {
    BrowserHarResponse {
        status: event.status.unwrap_or(0),
        status_text: reason.into(),
        headers: Vec::new(),
        content_text: None,
        route_result: event.route_result.clone().or_else(|| Some("abort".into())),
    }
}

fn status_text(status: u16) -> &'static str {
    match status {
        200..=299 => "OK",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        500..=599 => "Internal Server Error",
        _ => "",
    }
}
