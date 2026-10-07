//! Renew halfway through the server's TTL, never by assuming a fixed token lifetime.

use super::api::Lease;
use std::time::Duration;

pub(super) fn next(lease: Lease) -> Option<Duration> {
    if lease.ttl == 0 {
        tracing::info!("Vault token has no expiry; renewal not required");
        return None;
    }
    if !lease.renewable {
        tracing::warn!(
            ttl_seconds = lease.ttl,
            "Vault token is non-renewable or use-limited; obtain a renewable token before expiry"
        );
        return None;
    }
    Some(Duration::from_millis(
        lease.ttl.saturating_mul(500).min(3_600_000),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ttl_controls_schedule_and_unsupported_tokens_do_not_loop() {
        for (ttl, renewable, milliseconds) in [
            (1, true, Some(500)),
            (60, true, Some(30_000)),
            (u64::MAX, true, Some(3_600_000)),
            (0, true, None),
            (60, false, None),
        ] {
            assert_eq!(
                next(Lease { ttl, renewable }),
                milliseconds.map(Duration::from_millis)
            );
        }
    }
}
