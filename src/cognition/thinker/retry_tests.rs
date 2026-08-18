//! Tests for transient-failure classification.

use super::is_transient_http_error;

#[test]
fn treats_throttling_and_gateway_errors_as_transient() {
    for status in [429, 502, 503, 504] {
        assert!(is_transient_http_error(status), "{status} should retry");
    }
}

#[test]
fn treats_client_and_success_codes_as_final() {
    for status in [200, 400, 401, 404, 500] {
        assert!(
            !is_transient_http_error(status),
            "{status} should not retry"
        );
    }
}
