//! Staleness assessment for a stored Bedrock API key.
//!
//! Reads `api_key_expires_at` (epoch seconds) from a provider secret's extra
//! fields and decides whether the key needs a proactive silent refresh.

use crate::secrets::ProviderSecrets;
use chrono::Utc;

/// Refresh when the key expires within this many seconds (safety margin).
pub(crate) const REFRESH_MARGIN_SECS: i64 = 600;

/// Return `true` if the stored token is missing an expiry, already expired,
/// or expires within [`REFRESH_MARGIN_SECS`].
pub(crate) fn is_stale(secrets: &ProviderSecrets) -> bool {
    let Some(expires_at) = secrets
        .extra
        .get("api_key_expires_at")
        .and_then(serde_json::Value::as_i64)
    else {
        // No recorded expiry: don't second-guess a manually-set key.
        return false;
    };
    let now = Utc::now().timestamp();
    expires_at - now <= REFRESH_MARGIN_SECS
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn secret_expiring_in(secs: i64) -> ProviderSecrets {
        let mut s = ProviderSecrets::default();
        s.extra.insert(
            "api_key_expires_at".into(),
            json!(Utc::now().timestamp() + secs),
        );
        s
    }

    #[test]
    fn fresh_key_not_stale() {
        assert!(!is_stale(&secret_expiring_in(3600)));
    }

    #[test]
    fn near_expiry_is_stale() {
        assert!(is_stale(&secret_expiring_in(60)));
    }

    #[test]
    fn expired_is_stale() {
        assert!(is_stale(&secret_expiring_in(-10)));
    }

    #[test]
    fn no_expiry_not_stale() {
        assert!(!is_stale(&ProviderSecrets::default()));
    }
}
