//! Permit authenticated bootstrap, not another renewal, after a typed HTTP 429.
use super::rotation::Rotation;
use anyhow::Error;

#[derive(Debug, thiserror::Error)]
#[error("cookie_refresh_rejected: HTTP 429")]
pub(super) struct Throttled;

pub(super) fn cached_session_allowed(
    error: &Error,
    updated: u64,
    cookies: &[Rotation],
    instant: u64,
) -> bool {
    error.is::<Throttled>()
        && updated > 0
        && updated <= instant
        && cookies.iter().all(Rotation::valid)
        && cookies.iter().any(|cookie| {
            cookie.name == "__Secure-1PSIDTS"
                && cookie.expires > instant
                && !cookie.value.is_empty()
        })
}
