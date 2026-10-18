//! Existing route-permission regression.
use crate::routes::required_permission;
use axum::http::StatusCode;

#[test]
fn policy_unmapped_route_is_denied() {
    let permission = required_permission("/v1/new-route", "GET");
    assert_eq!(permission, Err(StatusCode::FORBIDDEN));
}
