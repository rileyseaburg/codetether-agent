//! HTTP implementation of RalphStateStore
//!
//! Posts state updates to the Python A2A server's Ralph API,
//! bridging the Rust agent's execution state to the Postgres-backed dashboard.

use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use tracing::{debug, warn};

use super::state_store::{RalphRunState, RalphRunSummary, RalphStateStore, StoryResultEntry};
use super::types::{Prd, ProgressEntry, RalphStatus};

/// HTTP-backed state store that talks to the Python A2A server
pub struct HttpStore {
    client: Client,
    base_url: String,
    api_key: Option<String>,
}

impl HttpStore {
    /// Create from environment variables.
    ///
    /// - `CODETETHER_API_URL` — base URL (default: `http://localhost:8080`)
    /// - `CODETETHER_API_KEY` — optional Bearer token
    pub fn from_env() -> Self {
        let base_url = std::env::var("CODETETHER_API_URL")
            .unwrap_or_else(|_| "http://localhost:8080".to_string());
        let api_key = std::env::var("CODETETHER_API_KEY")
            .ok()
            .filter(|s| !s.is_empty());

        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap_or_default(),
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key,
        }
    }

    /// Create with explicit base URL and optional API key
    pub fn new(base_url: &str, api_key: Option<String>) -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap_or_default(),
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key,
        }
    }

    fn apply_auth(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(ref key) = self.api_key {
            req.bearer_auth(key)
        } else {
            req
        }
    }

    fn status_str(status: RalphStatus) -> &'static str {
        match status {
            RalphStatus::Pending => "pending",
            RalphStatus::Running => "running",
            RalphStatus::Completed => "completed",
            RalphStatus::MaxIterations => "max_iterations",
            RalphStatus::Stopped => "stopped",
            RalphStatus::QualityFailed => "quality_failed",
        }
    }
}

#[async_trait]
impl RalphStateStore for HttpStore {
    async fn create_run(&self, state: &RalphRunState) -> anyhow::Result<()> {
        let url = format!("{}/v1/ralph/runs", self.base_url);
        let body = json!({
            "id": state.run_id,
            "prd": state.prd,
            "workspace_id": state.okr_id,
            "model": state.config.model,
            "status": Self::status_str(state.status),
            "max_iterations": state.max_iterations,
            "run_mode": if state.config.parallel_enabled { "parallel" } else { "sequential" },
            "max_parallel": state.config.max_concurrent_stories,
        });

        let req = self.apply_auth(self.client.post(&url)).json(&body);
        match req.send().await {
            Ok(resp) if resp.status().is_success() => {
                debug!(run_id = %state.run_id, "Created run in HTTP store");
            }
            Ok(resp) => {
                warn!(
                    run_id = %state.run_id,
                    status = %resp.status(),
                    "HTTP store create_run failed"
                );
            }
            Err(e) => {
                warn!(run_id = %state.run_id, error = %e, "HTTP store create_run error");
            }
        }
        Ok(())
    }

    async fn update_status(&self, run_id: &str, status: RalphStatus) -> anyhow::Result<()> {
        let url = format!("{}/v1/ralph/runs/{}", self.base_url, run_id);
        let mut body = json!({ "status": Self::status_str(status) });
        if status == RalphStatus::Running {
            body["started_at"] = json!(chrono::Utc::now().to_rfc3339());
        }

        let req = self.apply_auth(self.client.put(&url)).json(&body);
        if let Err(e) = req.send().await {
            warn!(run_id = %run_id, error = %e, "HTTP store update_status error");
        }
        Ok(())
    }

    async fn update_iteration(&self, run_id: &str, iteration: usize) -> anyhow::Result<()> {
        let url = format!("{}/v1/ralph/runs/{}", self.base_url, run_id);
        let body = json!({ "current_iteration": iteration });

        let req = self.apply_auth(self.client.put(&url)).json(&body);
        if let Err(e) = req.send().await {
            warn!(run_id = %run_id, error = %e, "HTTP store update_iteration error");
        }
        Ok(())
    }

    async fn record_story_result(
        &self,
        run_id: &str,
        result: &StoryResultEntry,
    ) -> anyhow::Result<()> {
        let url = format!(
            "{}/v1/ralph/runs/{}/stories/{}",
            self.base_url, run_id, result.story_id
        );
        let body = json!({
            "story_id": result.story_id,
            "title": result.title,
            "passed": result.passed,
            "iteration": result.iteration,
            "error": result.error,
        });

        let req = self.apply_auth(self.client.put(&url)).json(&body);
        if let Err(e) = req.send().await {
            warn!(run_id = %run_id, story = %result.story_id, error = %e, "HTTP store record_story_result error");
        }
        Ok(())
    }

    async fn append_progress(&self, run_id: &str, entry: &ProgressEntry) -> anyhow::Result<()> {
        // Append progress as a log entry via the update endpoint
        let url = format!("{}/v1/ralph/runs/{}", self.base_url, run_id);
        let body = json!({
            "logs": [entry],
        });

        let req = self.apply_auth(self.client.put(&url)).json(&body);
        if let Err(e) = req.send().await {
            warn!(run_id = %run_id, error = %e, "HTTP store append_progress error");
        }
        Ok(())
    }

    async fn update_prd(&self, run_id: &str, prd: &Prd) -> anyhow::Result<()> {
        let url = format!("{}/v1/ralph/runs/{}", self.base_url, run_id);
        let body = json!({ "prd": prd });

        let req = self.apply_auth(self.client.put(&url)).json(&body);
        if let Err(e) = req.send().await {
            warn!(run_id = %run_id, error = %e, "HTTP store update_prd error");
        }
        Ok(())
    }

    async fn set_error(&self, run_id: &str, error: &str) -> anyhow::Result<()> {
        let url = format!("{}/v1/ralph/runs/{}", self.base_url, run_id);
        let body = json!({ "error": error });

        let req = self.apply_auth(self.client.put(&url)).json(&body);
        if let Err(e) = req.send().await {
            warn!(run_id = %run_id, error = %e, "HTTP store set_error error");
        }
        Ok(())
    }

    async fn complete_run(&self, run_id: &str, status: RalphStatus) -> anyhow::Result<()> {
        let url = format!("{}/v1/ralph/runs/{}", self.base_url, run_id);
        let body = json!({
            "status": Self::status_str(status),
            "completed_at": chrono::Utc::now().to_rfc3339(),
        });

        let req = self.apply_auth(self.client.put(&url)).json(&body);
        if let Err(e) = req.send().await {
            warn!(run_id = %run_id, error = %e, "HTTP store complete_run error");
        }
        Ok(())
    }

    async fn get_run(&self, run_id: &str) -> anyhow::Result<Option<RalphRunState>> {
        let url = format!("{}/v1/ralph/runs/{}", self.base_url, run_id);
        let req = self.apply_auth(self.client.get(&url));

        match req.send().await {
            Ok(resp) if resp.status().is_success() => match resp.json::<RalphRunState>().await {
                Ok(state) => Ok(Some(state)),
                Err(e) => {
                    warn!(run_id = %run_id, error = %e, "HTTP store get_run parse error");
                    Ok(None)
                }
            },
            Ok(resp) if resp.status().as_u16() == 404 => Ok(None),
            Ok(resp) => {
                warn!(run_id = %run_id, status = %resp.status(), "HTTP store get_run failed");
                Ok(None)
            }
            Err(e) => {
                warn!(run_id = %run_id, error = %e, "HTTP store get_run error");
                Ok(None)
            }
        }
    }

    async fn list_runs(&self) -> anyhow::Result<Vec<RalphRunSummary>> {
        let url = format!("{}/v1/ralph/runs", self.base_url);
        let req = self.apply_auth(self.client.get(&url));

        match req.send().await {
            Ok(resp) if resp.status().is_success() => {
                match resp.json::<Vec<RalphRunSummary>>().await {
                    Ok(runs) => Ok(runs),
                    Err(e) => {
                        warn!(error = %e, "HTTP store list_runs parse error");
                        Ok(Vec::new())
                    }
                }
            }
            Ok(resp) => {
                warn!(status = %resp.status(), "HTTP store list_runs failed");
                Ok(Vec::new())
            }
            Err(e) => {
                warn!(error = %e, "HTTP store list_runs error");
                Ok(Vec::new())
            }
        }
    }
}
