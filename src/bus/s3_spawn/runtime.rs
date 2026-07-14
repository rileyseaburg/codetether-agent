//! Dedicated executor bounds for the MinIO client's async-std dependency.
//!
//! `CODETETHER_S3_RUNTIME_THREADS` overrides the two-thread default and is
//! clamped to eight threads.

use std::sync::Once;

const ENV_THREADS: &str = "CODETETHER_S3_RUNTIME_THREADS";

pub(super) fn initialize() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let threads = thread_count();
        async_global_executor::init_with_config(
            async_global_executor::GlobalExecutorConfig::default()
                .with_min_threads(threads)
                .with_max_threads(threads)
                .with_thread_name_fn(|| "codetether-s3".to_string()),
        );
        tracing::info!(threads, "Initialized bounded S3 executor");
    });
}

fn thread_count() -> usize {
    parse(std::env::var(ENV_THREADS).ok().as_deref())
}

fn parse(raw: Option<&str>) -> usize {
    raw.and_then(|value| value.trim().parse().ok())
        .unwrap_or(2)
        .clamp(1, 8)
}

#[cfg(test)]
mod tests {
    #[test]
    fn thread_count_defaults_and_clamps() {
        assert_eq!(super::parse(None), 2);
        assert_eq!(super::parse(Some("0")), 1);
        assert_eq!(super::parse(Some("99")), 8);
        assert_eq!(super::parse(Some("invalid")), 2);
    }
}
