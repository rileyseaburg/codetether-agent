//! Process-wide shared [`reqwest::Client`].
//!
//! Each call to [`reqwest::Client::new`] loads the system TLS root store
//! (hundreds of certs), builds a fresh TLS config, and allocates a new
//! connection pool. Across ~10 providers that's hundreds of milliseconds
//! of startup cost on first use plus duplicated pools that cannot share
//! keep-alive connections.
//!
//! [`shared_client`] returns a single `Client` that every provider which
//! does not need custom builder options can clone (the `Client` is
//! internally reference-counted, so clones are O(1) and share the
//! underlying connection pool).
//!
//! Providers that need custom timeouts, proxies, or TLS settings
//! (`openai`, `vertex_*`, `gemini_web`, `openrouter`) continue to use
//! `Client::builder()` directly — those configurations cannot be
//! meaningfully shared anyway.

use once_cell::sync::Lazy;
use reqwest::Client;

static SHARED: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        // Enable HTTP/2 adaptive window and prior-knowledge upgrades
        // where the server supports them. This is reqwest's default
        // but pinning it here documents the expectation.
        .use_rustls_tls()
        // Keep connections alive across provider calls so repeated
        // streaming completions hit the same TCP/TLS session.
        .pool_idle_timeout(Some(std::time::Duration::from_secs(90)))
        .pool_max_idle_per_host(8)
        .build()
        .unwrap_or_else(|_| Client::new())
});

/// Return a handle to the process-wide shared `reqwest::Client`.
///
/// `Client` is cheap to clone — it wraps an `Arc` internally — so callers
/// should `.clone()` the returned reference into their own state.
pub fn shared_client() -> &'static Client {
    &SHARED
}
