//! Effective-expiry computation for minted Bedrock API keys.

use chrono::{DateTime, Duration, Utc};

/// Resolve when the minted key effectively expires.
///
/// Returns the earlier of the token's own TTL and the underlying credential
/// expiry, because an SSO/STS session dying invalidates the key before its
/// nominal TTL elapses.
pub(crate) fn effective(
    lifetime_secs: u64,
    cred_expiration: Option<DateTime<Utc>>,
) -> DateTime<Utc> {
    let token_expires_at = Utc::now() + Duration::seconds(lifetime_secs as i64);
    match cred_expiration {
        Some(cred) if cred < token_expires_at => cred,
        _ => token_expires_at,
    }
}

#[cfg(test)]
mod tests {
    use super::effective;
    use chrono::{Duration, Utc};

    #[test]
    fn clamps_to_earlier_credential_expiry() {
        let cred = Utc::now() + Duration::seconds(600);
        let got = effective(43200, Some(cred));
        assert_eq!(got, cred);
    }

    #[test]
    fn uses_token_ttl_when_credential_outlives_it() {
        let cred = Utc::now() + Duration::seconds(100_000);
        let got = effective(3600, Some(cred));
        assert!(got < cred);
    }

    #[test]
    fn uses_token_ttl_when_no_credential_expiry() {
        let before = Utc::now() + Duration::seconds(3600);
        let got = effective(3600, None);
        assert!(got >= before - Duration::seconds(2));
    }
}
