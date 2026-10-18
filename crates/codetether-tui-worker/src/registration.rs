//! Application-specific worker registration boundary.
use anyhow::Result;
use reqwest::Client;
/// Registration request using caller-owned connection details.
/// # Examples
/// ```no_run
/// let client = reqwest::Client::new(); let token = None; let paths = Vec::new();
/// let request = codetether_tui_worker::RegistrationRequest { client: &client, server: "http://localhost", token: &token, worker_id: "worker", worker_name: "tui", codebases: &paths };
/// assert_eq!(request.worker_name, "tui");
/// ```
pub struct RegistrationRequest<'a> {
    /// Shared HTTP client.
    pub client: &'a Client,
    /// Control-plane base URL.
    pub server: &'a str,
    /// Optional bearer token.
    pub token: &'a Option<String>,
    /// Generated worker identifier.
    pub worker_id: &'a str,
    /// Configured or process-derived worker name.
    pub worker_name: &'a str,
    /// Workspace paths advertised by the worker.
    pub codebases: &'a [String],
}
/// Supply application-specific worker metadata during registration.
/// # Implementors
/// Delegate to the application's canonical worker registration boundary.
#[async_trait::async_trait]
pub trait Registration: Send + Sync + 'static {
    /// Register once before heartbeat and SSE startup.
    /// # Errors
    /// Return the underlying registration error; the bridge logs and continues.
    async fn register(request: RegistrationRequest<'_>) -> Result<()>;
}
