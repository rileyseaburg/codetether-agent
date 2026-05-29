//! Persistent-worker heartbeat configuration helpers.

pub(super) fn persistent_worker_enabled() -> bool {
    std::env::var("PERSISTENT_WORKER_ENABLED")
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false)
}

pub(super) fn persistent_worker_lease_seconds() -> u64 {
    std::env::var("PERSISTENT_WORKER_LEASE_SECONDS")
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(3600)
}
