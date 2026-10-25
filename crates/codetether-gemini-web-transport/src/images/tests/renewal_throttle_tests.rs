//! Throttling never accepts expired cookies or masks other renewal failures.
use crate::images::{
    renewal_throttle::{Throttled, cached_session_allowed},
    rotation::Rotation,
};

fn token(expires: u64) -> Rotation {
    Rotation {
        name: "__Secure-1PSIDTS".into(),
        value: "fixture".into(),
        expires,
    }
}

#[test]
fn typed_throttle_allows_unexpired_cache_beyond_the_renewal_interval() {
    let error = anyhow::Error::new(Throttled).context("renewal");
    assert!(cached_session_allowed(&error, 1, &[token(121)], 120));
    assert_eq!(
        error.root_cause().to_string(),
        "cookie_refresh_rejected: HTTP 429"
    );
}

#[test]
fn expired_missing_and_future_cache_never_bypass_throttling() {
    let error = anyhow::Error::new(Throttled);
    assert!(!cached_session_allowed(&error, 1, &[token(120)], 120));
    assert!(!cached_session_allowed(&error, 0, &[token(121)], 120));
    assert!(!cached_session_allowed(&error, 121, &[token(121)], 120));
    assert!(!cached_session_allowed(&error, 1, &[], 120));
    let mut empty = token(121);
    empty.value.clear();
    assert!(!cached_session_allowed(&error, 1, &[empty], 120));
    let mut invalid = token(121);
    invalid.name = "untrusted".into();
    assert!(!cached_session_allowed(&error, 1, &[invalid], 120));
}

#[test]
fn rejection_and_untyped_throttle_errors_are_not_hidden() {
    for message in [
        "cookie_refresh_rejected: HTTP 401",
        "cookie_refresh_rejected: HTTP 429",
        "image_transport_failed",
    ] {
        let error = anyhow::anyhow!(message);
        assert!(!cached_session_allowed(&error, 1, &[token(121)], 120));
    }
}
