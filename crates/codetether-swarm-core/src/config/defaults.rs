//! Serde and builder defaults for swarm execution settings.

pub(super) fn default_k8s_pod_budget() -> usize {
    8
}

pub(super) fn default_max_retries() -> u32 {
    3
}

pub(super) fn default_base_delay_ms() -> u64 {
    500
}

pub(super) fn default_max_delay_ms() -> u64 {
    30_000
}
