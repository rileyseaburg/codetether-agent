//! Shared bounded-time OPA HTTP connection pool.
use reqwest::Client;
use std::{sync::LazyLock, time::Duration};

// ─── Shared HTTP client ──────────────────────────────────────────

pub(crate) static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(2))
        .pool_max_idle_per_host(4)
        .build()
        .expect("failed to build reqwest client")
});
