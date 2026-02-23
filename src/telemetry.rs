//! Telemetry module for monitoring and observability
//!
//! Provides telemetry capabilities for tracking agent performance and behavior.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use once_cell::sync::Lazy;

// Global telemetry counters
pub static TOKEN_USAGE: Lazy<Arc<AtomicTokenCounter>> = Lazy::new(|| {
    Arc::new(AtomicTokenCounter::new())
});

pub static TOOL_EXECUTIONS: Lazy<Arc<AtomicToolCounter>> = Lazy::new(|| {
    Arc::new(AtomicToolCounter::new())
});

pub static PROVIDER_METRICS: Lazy<Arc<ProviderMetrics>> = Lazy::new(|| {
    Arc::new(ProviderMetrics::new())
});

/// Token totals structure
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TokenTotals {
    pub input: u64,
    pub output: u64,
}

impl TokenTotals {
    pub fn new(input: u64, output: u64) -> Self {
        Self { input, output }
    }

    pub fn total(&self) -> u64 {
        self.input + self.output
    }
}

/// Global token snapshot
#[derive(Debug, Clone, Default)]
pub struct GlobalTokenSnapshot {
    pub input: u64,
    pub output: u64,
    pub total: TokenTotals,
    pub totals: TokenTotals,
    pub request_count: u64,
}

impl GlobalTokenSnapshot {
    pub fn new(input: u64, output: u64, _total: u64) -> Self {
        Self {
            input,
            output,
            total: TokenTotals::new(input, output),
            totals: TokenTotals::new(input, output),
            request_count: 0,
        }
    }

    pub fn summary(&self) -> String {
        format!("{} total tokens ({} input, {} output)", self.totals.total(), self.input, self.output)
    }
}

/// Atomic token counter
#[derive(Debug)]
pub struct AtomicTokenCounter {
    prompt_tokens: AtomicU64,
    completion_tokens: AtomicU64,
    total_tokens: AtomicU64,
    request_count: AtomicU64,
    model_usage: Mutex<HashMap<String, (u64, u64)>>,
}

impl AtomicTokenCounter {
    pub fn new() -> Self {
        Self {
            prompt_tokens: AtomicU64::new(0),
            completion_tokens: AtomicU64::new(0),
            total_tokens: AtomicU64::new(0),
            request_count: AtomicU64::new(0),
            model_usage: Mutex::new(HashMap::new()),
        }
    }

    pub fn record(&self, prompt: u64, completion: u64) {
        self.prompt_tokens.fetch_add(prompt, Ordering::Relaxed);
        self.completion_tokens.fetch_add(completion, Ordering::Relaxed);
        self.total_tokens.fetch_add(prompt + completion, Ordering::Relaxed);
        self.request_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get(&self) -> (u64, u64, u64) {
        (
            self.prompt_tokens.load(Ordering::Relaxed),
            self.completion_tokens.load(Ordering::Relaxed),
            self.total_tokens.load(Ordering::Relaxed),
        )
    }

    pub fn record_model_usage(&self, model: &str, prompt: u64, completion: u64) {
        tracing::debug!(model, prompt, completion, "Recording model usage");
        self.record(prompt, completion);
        
        // Also track per-model usage
        if let Ok(mut usage) = self.model_usage.try_lock() {
            let entry = usage.entry(model.to_string()).or_insert((0, 0));
            entry.0 += prompt;
            entry.1 += completion;
        }
    }

    pub fn global_snapshot(&self) -> GlobalTokenSnapshot {
        let (prompt, completion, total) = self.get();
        let mut snapshot = GlobalTokenSnapshot::new(prompt, completion, total);
        snapshot.request_count = self.request_count.load(Ordering::Relaxed);
        snapshot
    }

    pub fn model_snapshots(&self) -> Vec<TokenUsageSnapshot> {
        if let Ok(usage) = self.model_usage.try_lock() {
            usage.iter().map(|(name, (input, output))| {
                TokenUsageSnapshot {
                    name: name.clone(),
                    prompt_tokens: *input,
                    completion_tokens: *output,
                    total_tokens: input + output,
                    totals: TokenTotals::new(*input, *output),
                    timestamp: Utc::now(),
                    request_count: 0, // Default value, per-model request count not tracked yet
                }
            }).collect()
        } else {
            Vec::new()
        }
    }
}

impl Default for AtomicTokenCounter {
    fn default() -> Self {
        Self::new()
    }
}

/// Atomic tool execution counter
#[derive(Debug)]
pub struct AtomicToolCounter {
    count: AtomicU64,
    failures: AtomicU64,
}

impl AtomicToolCounter {
    pub fn new() -> Self {
        Self {
            count: AtomicU64::new(0),
            failures: AtomicU64::new(0),
        }
    }

    pub fn record(&self, success: bool) {
        self.count.fetch_add(1, Ordering::Relaxed);
        if !success {
            self.failures.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn get(&self) -> (u64, u64) {
        (
            self.count.load(Ordering::Relaxed),
            self.failures.load(Ordering::Relaxed),
        )
    }
}

impl Default for AtomicToolCounter {
    fn default() -> Self {
        Self::new()
    }
}

/// Tool execution record for telemetry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolExecution {
    pub tool_name: String,
    pub timestamp: DateTime<Utc>,
    pub duration_ms: u64,
    pub success: bool,
    pub error: Option<String>,
    pub tokens_used: Option<u64>,
    pub session_id: Option<String>,
    pub input: Option<serde_json::Value>,
    #[serde(default)]
    pub file_changes: Vec<FileChange>,
}

impl ToolExecution {
    /// Start a new tool execution
    pub fn start(tool_name: &str, input: serde_json::Value) -> Self {
        Self {
            tool_name: tool_name.to_string(),
            timestamp: Utc::now(),
            duration_ms: 0,
            success: false,
            error: None,
            tokens_used: None,
            session_id: None,
            input: Some(input),
            file_changes: Vec::new(),
        }
    }

    /// Add a file change
    pub fn add_file_change(&mut self, change: FileChange) {
        self.file_changes.push(change);
    }

    /// Add session ID
    pub fn with_session(mut self, session_id: String) -> Self {
        self.session_id = Some(session_id);
        self
    }

    /// Mark as complete
    pub fn complete(&mut self, success: bool, duration_ms: u64) {
        self.success = success;
        self.duration_ms = duration_ms;
    }

    /// Mark as failed with error
    pub fn fail(&mut self, error: String, duration_ms: u64) {
        self.success = false;
        self.error = Some(error);
        self.duration_ms = duration_ms;
    }

    /// Complete with success
    pub fn complete_success(mut self, _output: String, duration: std::time::Duration) -> Self {
        self.success = true;
        self.duration_ms = duration.as_millis() as u64;
        self
    }

    /// Complete with error
    pub fn complete_error(mut self, error: String, duration: std::time::Duration) -> Self {
        self.success = false;
        self.error = Some(error);
        self.duration_ms = duration.as_millis() as u64;
        self
    }
}

/// File change record for telemetry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChange {
    pub path: String,
    pub operation: String,
    pub timestamp: DateTime<Utc>,
    pub size_bytes: Option<u64>,
    pub line_range: Option<(u32, u32)>,
    pub diff: Option<String>,
}

impl FileChange {
    /// Create a file read record
    pub fn read(path: &str, line_range: Option<(u32, u32)>) -> Self {
        Self {
            path: path.to_string(),
            operation: "read".to_string(),
            timestamp: Utc::now(),
            size_bytes: None,
            line_range,
            diff: None,
        }
    }

    /// Create a file create record
    pub fn create(path: &str, content: &str) -> Self {
        Self {
            path: path.to_string(),
            operation: "create".to_string(),
            timestamp: Utc::now(),
            size_bytes: Some(content.len() as u64),
            line_range: None,
            diff: None,
        }
    }

    /// Create a file modify record
    pub fn modify(path: &str, old_content: &str, new_content: &str) -> Self {
        Self {
            path: path.to_string(),
            operation: "modify".to_string(),
            timestamp: Utc::now(),
            size_bytes: Some(new_content.len() as u64),
            line_range: None,
            diff: Some(format!("-{} bytes +{} bytes", old_content.len(), new_content.len())),
        }
    }

    /// Create a file modify record with diff
    pub fn modify_with_diff(path: &str, diff: &str, new_size: usize) -> Self {
        Self {
            path: path.to_string(),
            operation: "modify".to_string(),
            timestamp: Utc::now(),
            size_bytes: Some(new_size as u64),
            line_range: None,
            diff: Some(diff.to_string()),
        }
    }
    pub fn summary(&self) -> String {
        format!("{} ({})", self.path, self.operation)
    }
}

/// Provider request record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderRequestRecord {
    pub provider: String,
    pub model: String,
    pub timestamp: DateTime<Utc>,
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub latency_ms: u64,
    pub ttft_ms: Option<u64>,
    pub success: bool,
}

impl ProviderRequestRecord {
    /// Calculate tokens per second
    pub fn tokens_per_second(&self) -> f64 {
        if self.latency_ms == 0 {
            return 0.0;
        }
        (self.output_tokens as f64) / (self.latency_ms as f64 / 1000.0)
    }
}

/// Token usage snapshot (per-model)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsageSnapshot {
    pub name: String,
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
    pub totals: TokenTotals,
    pub timestamp: DateTime<Utc>,
    pub request_count: u64,
}

impl TokenUsageSnapshot {
    pub fn current() -> Self {
        let (prompt, comp, total) = TOKEN_USAGE.get();
        Self {
            name: "global".to_string(),
            prompt_tokens: prompt,
            completion_tokens: comp,
            total_tokens: total,
            totals: TokenTotals::new(prompt, comp),
            timestamp: Utc::now(),
            request_count: 0,
        }
    }

    pub fn summary(&self) -> String {
        format!("{} total tokens ({} input, {} output)", self.totals.total(), self.prompt_tokens, self.completion_tokens)
    }
}

/// Token counts structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenCounts {
    pub input_tokens: u64,
    pub output_tokens: u64,
}

impl TokenCounts {
    pub fn new(input_tokens: u64, output_tokens: u64) -> Self {
        Self {
            input_tokens,
            output_tokens,
        }
    }
}

/// Context limit info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextLimit {
    pub max_tokens: u64,
    pub used_tokens: u64,
    pub remaining_tokens: u64,
    pub percentage_used: f64,
    /// Alias for percentage_used (for API compatibility)
    pub percentage: f64,
}

impl ContextLimit {
    pub fn new(used_tokens: u64, max_tokens: u64) -> Self {
        let remaining = max_tokens.saturating_sub(used_tokens);
        let percentage = if max_tokens > 0 {
            (used_tokens as f64 / max_tokens as f64) * 100.0
        } else {
            0.0
        };
        Self {
            max_tokens,
            used_tokens,
            remaining_tokens: remaining,
            percentage_used: percentage,
            percentage,
        }
    }

    /// Get percentage (alias for percentage_used)
    pub fn percentage(&self) -> f64 {
        self.percentage_used
    }
}

/// Cost estimate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostEstimate {
    pub input_cost: f64,
    pub output_cost: f64,
    pub total_cost: f64,
    pub currency: String,
}

impl Default for CostEstimate {
    fn default() -> Self {
        Self {
            input_cost: 0.0,
            output_cost: 0.0,
            total_cost: 0.0,
            currency: "USD".to_string(),
        }
    }
}

impl CostEstimate {
    pub fn from_tokens(tokens: &TokenCounts, input_price: f64, output_price: f64) -> Self {
        let input_cost = (tokens.input_tokens as f64 / 1_000_000.0) * input_price;
        let output_cost = (tokens.output_tokens as f64 / 1_000_000.0) * output_price;
        Self {
            input_cost,
            output_cost,
            total_cost: input_cost + output_cost,
            currency: "USD".to_string(),
        }
    }

    pub fn format_currency(&self) -> String {
        format!("${:.4}", self.total_cost)
    }

    pub fn format_smart(&self) -> String {
        if self.total_cost < 0.01 {
            format!("${:.4}", self.total_cost)
        } else if self.total_cost < 1.0 {
            format!("${:.2}", self.total_cost)
        } else {
            format!("${:.2}", self.total_cost)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderSnapshot {
    pub provider: String,
    pub request_count: usize,
    pub total_input_tokens: u64,
    pub total_output_tokens: u64,
    pub avg_tps: f64,
    pub avg_latency_ms: f64,
    pub p50_tps: f64,
    pub p50_latency_ms: f64,
    pub p95_tps: f64,
    pub p95_latency_ms: f64,
}

/// Provider metrics
#[derive(Debug, Default)]
pub struct ProviderMetrics {
    requests: Mutex<Vec<ProviderRequestRecord>>,
}

impl ProviderMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn record(&self, record: ProviderRequestRecord) {
        let mut requests = self.requests.lock().await;
        requests.push(record);
        // Keep only last 1000 requests
        if requests.len() > 1000 {
            requests.remove(0);
        }
    }

    pub async fn get_recent(&self, limit: usize) -> Vec<ProviderRequestRecord> {
        let requests = self.requests.lock().await;
        requests.iter().rev().take(limit).cloned().collect()
    }

    pub fn all_snapshots(&self) -> Vec<ProviderSnapshot> {
        let requests = match self.requests.try_lock() {
            Ok(guard) => guard.clone(),
            Err(_) => return Vec::new(),
        };

        if requests.is_empty() {
            return Vec::new();
        }

        let mut by_provider: HashMap<String, Vec<ProviderRequestRecord>> = HashMap::new();
        for req in requests {
            by_provider.entry(req.provider.clone()).or_default().push(req);
        }

        let mut snapshots = Vec::new();
        for (provider, mut reqs) in by_provider {
            if reqs.is_empty() {
                continue;
            }

            let request_count = reqs.len();
            let total_input_tokens: u64 = reqs.iter().map(|r| r.input_tokens).sum();
            let total_output_tokens: u64 = reqs.iter().map(|r| r.output_tokens).sum();
            let total_latency: u64 = reqs.iter().map(|r| r.latency_ms).sum();
            
            let avg_latency_ms = total_latency as f64 / request_count as f64;
            
            let mut tps_values: Vec<f64> = reqs.iter().map(|r| r.tokens_per_second()).collect();
            tps_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            
            let mut latency_values: Vec<f64> = reqs.iter().map(|r| r.latency_ms as f64).collect();
            latency_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

            let p50_idx = (request_count as f64 * 0.50) as usize;
            let p95_idx = (request_count as f64 * 0.95) as usize;

            let p50_tps = tps_values.get(p50_idx).cloned().unwrap_or(0.0);
            let p95_tps = tps_values.get(p95_idx).cloned().unwrap_or(0.0);
            
            let p50_latency_ms = latency_values.get(p50_idx).cloned().unwrap_or(0.0);
            let p95_latency_ms = latency_values.get(p95_idx).cloned().unwrap_or(0.0);

            let avg_tps = tps_values.iter().sum::<f64>() / request_count as f64;

            snapshots.push(ProviderSnapshot {
                provider,
                request_count,
                total_input_tokens,
                total_output_tokens,
                avg_tps,
                avg_latency_ms,
                p50_tps,
                p50_latency_ms,
                p95_tps,
                p95_latency_ms,
            });
        }

        snapshots
    }
}

/// Record a persistent telemetry entry
pub fn record_persistent(category: &str, data: &serde_json::Value) -> Result<()> {
    tracing::debug!(category, data = ?data, "Recording persistent telemetry");
    // In a real implementation, this would write to a persistent store
    Ok(())
}

/// Swarm telemetry collector
#[derive(Debug, Default)]
pub struct SwarmTelemetryCollector {
    task_id: Mutex<Option<String>>,
    agent_count: Mutex<usize>,
    completed: Mutex<usize>,
    total: Mutex<usize>,
    start_time: Mutex<Option<DateTime<Utc>>>,
}

impl SwarmTelemetryCollector {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn start_swarm(&self, task_id: &str, agent_count: usize, _strategy: &str) {
        let mut id = self.task_id.lock().await;
        *id = Some(task_id.to_string());
        let mut count = self.agent_count.lock().await;
        *count = agent_count;
        let mut start = self.start_time.lock().await;
        *start = Some(Utc::now());
        tracing::info!(task_id, agent_count, "Swarm started");
    }

    pub async fn record_progress(&self, completed: usize, total: usize) {
        let mut c = self.completed.lock().await;
        *c = completed;
        let mut t = self.total.lock().await;
        *t = total;
    }

    pub async fn record_swarm_latency(&self, _label: &str, duration: std::time::Duration) {
        tracing::debug!(label = _label, duration_ms = duration.as_millis(), "Swarm latency recorded");
    }

    pub async fn complete_swarm(&self, success: bool) -> TelemetryMetrics {
        let start = self.start_time.lock().await;
        let duration = start.map(|s| (Utc::now() - s).num_milliseconds() as u64).unwrap_or(0);
        drop(start);
        
        let completed = *self.completed.lock().await;
        let total = *self.total.lock().await;
        
        tracing::info!(
            success,
            completed,
            total,
            duration_ms = duration,
            "Swarm completed"
        );
        
        TelemetryMetrics {
            tool_invocations: total as u64,
            successful_operations: if success { completed as u64 } else { 0 },
            failed_operations: if !success { (total.saturating_sub(completed)) as u64 } else { 0 },
            total_tokens: 0,
            avg_latency_ms: duration as f64,
        }
    }
}

/// Telemetry metrics for agent operations
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TelemetryMetrics {
    /// Number of tool invocations
    pub tool_invocations: u64,
    /// Number of successful operations
    pub successful_operations: u64,
    /// Number of failed operations
    pub failed_operations: u64,
    /// Total tokens consumed
    pub total_tokens: u64,
    /// Average latency in milliseconds
    pub avg_latency_ms: f64,
}

/// Telemetry tracker for the agent
#[derive(Debug)]
pub struct Telemetry {
    metrics: Mutex<TelemetryMetrics>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl Telemetry {
    /// Create a new telemetry instance
    pub fn new() -> Self {
        Self {
            metrics: Mutex::new(TelemetryMetrics::default()),
            metadata: HashMap::new(),
        }
    }

    /// Record a tool invocation
    pub async fn record_tool_invocation(&self, success: bool, latency_ms: u64, tokens: u64) {
        let mut metrics = self.metrics.lock().await;
        metrics.tool_invocations += 1;
        if success {
            metrics.successful_operations += 1;
        } else {
            metrics.failed_operations += 1;
        }
        metrics.total_tokens += tokens;
        // Simple rolling average
        let n = metrics.tool_invocations as f64;
        metrics.avg_latency_ms = metrics.avg_latency_ms * (n - 1.0) / n + latency_ms as f64 / n;
    }

    /// Get current metrics
    pub async fn get_metrics(&self) -> TelemetryMetrics {
        self.metrics.lock().await.clone()
    }

    /// Start a swarm operation (placeholder)
    pub async fn start_swarm(&self, _task_id: &str, _agent_count: usize) {
        // Placeholder for swarm telemetry
    }

    /// Record swarm progress (placeholder)
    pub async fn record_swarm_progress(&self, _task_id: &str, _completed: usize, _total: usize) {
        // Placeholder for swarm telemetry
    }

    /// Complete a swarm operation (placeholder)
    pub async fn complete_swarm(&self, _success: bool) -> TelemetryMetrics {
        self.metrics.lock().await.clone()
    }
}

impl Default for Telemetry {
    fn default() -> Self {
        Self::new()
    }
}

/// Stats about persistent telemetry
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PersistentStats {
    pub stats: PersistentStatsInner,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PersistentStatsInner {
    pub total_input_tokens: u64,
    pub total_output_tokens: u64,
    pub total_requests: u64,
    pub executions_by_tool: HashMap<String, u64>,
    pub files_modified: HashMap<String, u64>,
}

impl PersistentStats {
    pub fn recent(&self, _limit: usize) -> Vec<ToolExecution> {
        Vec::new()
    }

    pub fn all_file_changes(&self) -> Vec<(String, FileChange)> {
        Vec::new()
    }

    pub fn by_tool(&self, _tool_name: &str) -> Vec<ToolExecution> {
        Vec::new()
    }

    pub fn by_file(&self, _file_path: &str) -> Vec<ToolExecution> {
        Vec::new()
    }

    pub fn summary(&self) -> String {
        "0 total executions".to_string()
    }
}

/// Get persistent stats
pub fn get_persistent_stats() -> PersistentStats {
    PersistentStats::default()
}
