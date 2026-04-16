//! Worker identity and concurrency helpers.
//!
//! These helpers keep startup defaults small and deterministic while the main
//! worker loop focuses on task execution.
//!
//! # Examples
//!
//! ```ignore
//! let worker_id = generate_worker_id();
//! assert!(worker_id.starts_with("wrk_"));
//! ```

/// Generates a fresh worker identifier for registration and heartbeats.
///
/// The identifier embeds a timestamp and random suffix so concurrent workers do
/// not collide even when they start at nearly the same time.
///
/// # Examples
///
/// ```ignore
/// let worker_id = generate_worker_id();
/// assert!(worker_id.starts_with("wrk_"));
/// ```
pub fn generate_worker_id() -> String {
    format!(
        "wrk_{}_{:x}",
        chrono::Utc::now().timestamp(),
        rand::random::<u64>()
    )
}

/// Resolves the effective worker identifier from environment or fallback.
///
/// `CODETETHER_WORKER_ID` takes precedence, followed by `A2A_WORKER_ID`, then
/// a generated identifier when neither variable is set.
///
/// # Examples
///
/// ```ignore
/// let worker_id = resolve_worker_id();
/// assert!(!worker_id.trim().is_empty());
/// ```
pub(super) fn resolve_worker_id() -> String {
    ["CODETETHER_WORKER_ID", "A2A_WORKER_ID"]
        .into_iter()
        .find_map(|key| {
            std::env::var(key)
                .ok()
                .map(|value| value.trim().to_string())
        })
        .filter(|value| !value.is_empty())
        .unwrap_or_else(generate_worker_id)
}

/// Clamps worker concurrency to at least one active task.
///
/// This prevents misconfiguration from disabling task processing entirely.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(normalize_max_concurrent_tasks(0), 1);
/// ```
pub(super) fn normalize_max_concurrent_tasks(max_concurrent_tasks: usize) -> usize {
    max_concurrent_tasks.max(1)
}
