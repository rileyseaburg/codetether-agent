//! Runtime policy for Kubernetes mutation leases.

const DEFAULT_LEASE_SECONDS: i32 = 30;

pub fn lease_seconds() -> i32 {
    std::env::var("CODETETHER_K8S_LEASE_SECONDS")
        .ok()
        .and_then(|value| value.parse().ok())
        .filter(|value| *value >= 6)
        .unwrap_or(DEFAULT_LEASE_SECONDS)
}
