//! CI status watcher for auto-pilot PRs.

#[allow(unused_imports)]
use super::autopilot::PrLifecycle;

/// CI check result.
#[derive(Debug, Clone)]
pub struct CiCheckResult {
    pub all_green: bool,
    pub failed_jobs: Vec<String>,
    pub pending_jobs: Vec<String>,
}

impl CiCheckResult {
    pub fn from_github_checks(data: &serde_json::Value) -> Self {
        let statuses = data.get("statuses").and_then(|s| s.as_array());
        let checks = data.get("check_suites").and_then(|c| c.as_array());
        let all_results: Vec<&str> = [statuses, checks]
            .into_iter()
            .flatten()
            .flat_map(|arr| arr.iter())
            .filter_map(|v| v.get("state").and_then(|s| s.as_str()))
            .collect();
        let failed: Vec<String> = all_results.iter()
            .filter(|&&s| s == "failure" || s == "error")
            .map(|s| s.to_string())
            .collect();
        let pending: Vec<String> = all_results.iter()
            .filter(|&&s| s == "pending" || s == "in_progress")
            .map(|s| s.to_string())
            .collect();
        Self { all_green: failed.is_empty() && pending.is_empty() && !all_results.is_empty(), failed_jobs: failed, pending_jobs: pending }
    }
}
