//! Serde defaults for S3 batching configuration.

pub(super) fn default_prefix() -> String {
    "training/".to_string()
}

pub(super) fn default_batch_size() -> usize {
    100
}

pub(super) fn default_flush_interval_secs() -> u64 {
    30
}
