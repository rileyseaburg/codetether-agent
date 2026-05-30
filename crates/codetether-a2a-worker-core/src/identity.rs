//! Worker identity and concurrency helpers.

pub fn generate_worker_id() -> String {
    format!(
        "wrk_{}_{:x}",
        chrono::Utc::now().timestamp(),
        rand::random::<u64>()
    )
}

pub fn resolve_worker_id() -> String {
    env_worker_id().unwrap_or_else(generate_worker_id)
}

pub fn normalize_max_concurrent_tasks(max_concurrent_tasks: usize) -> usize {
    max_concurrent_tasks.max(1)
}

fn env_worker_id() -> Option<String> {
    ["CODETETHER_WORKER_ID", "A2A_WORKER_ID"]
        .into_iter()
        .find_map(|key| {
            std::env::var(key)
                .ok()
                .map(|value| value.trim().to_string())
        })
        .filter(|value| !value.is_empty())
}
