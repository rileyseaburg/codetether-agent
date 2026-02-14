//! Telemetry and usage tracking
//!
//! Tracks token usage, costs, tool executions, file changes, and other metrics
//! for monitoring agent performance and providing audit trails.

use parking_lot::RwLock;
use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime};

// ============================================================================
// Tool Execution Tracking
// ============================================================================

/// Unique identifier for a tool execution
pub type ToolExecId = u64;

/// A single tool execution record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolExecution {
    /// Unique execution ID
    pub id: ToolExecId,
    /// Tool name/identifier
    pub tool_name: String,
    /// Full input arguments (JSON)
    pub input: serde_json::Value,
    /// Output result
    pub output: Option<String>,
    /// Whether execution succeeded
    pub success: bool,
    /// Error message if failed
    pub error: Option<String>,
    /// Execution start time (Unix timestamp ms)
    pub started_at: u64,
    /// Duration in milliseconds
    pub duration_ms: u64,
    /// Files affected by this tool
    pub files_affected: Vec<FileChange>,
    /// Token usage for this execution (if applicable)
    pub tokens: Option<TokenCounts>,
    /// Parent execution ID (for nested/sub-agent calls)
    pub parent_id: Option<ToolExecId>,
    /// Session or agent ID
    pub session_id: Option<String>,
    /// Model used (if applicable)
    pub model: Option<String>,
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

impl ToolExecution {
    /// Create a new tool execution record (call this when starting execution)
    pub fn start(tool_name: impl Into<String>, input: serde_json::Value) -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);

        let started_at = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);

        Self {
            id,
            tool_name: tool_name.into(),
            input,
            output: None,
            success: false,
            error: None,
            started_at,
            duration_ms: 0,
            files_affected: Vec::new(),
            tokens: None,
            parent_id: None,
            session_id: None,
            model: None,
            metadata: HashMap::new(),
        }
    }

    /// Complete the execution with success
    pub fn complete_success(mut self, output: String, duration: Duration) -> Self {
        self.output = Some(output);
        self.success = true;
        self.duration_ms = duration.as_millis() as u64;
        self
    }

    /// Complete the execution with an error
    pub fn complete_error(mut self, error: String, duration: Duration) -> Self {
        self.error = Some(error);
        self.success = false;
        self.duration_ms = duration.as_millis() as u64;
        self
    }

    /// Add a file change to this execution
    pub fn add_file_change(&mut self, change: FileChange) {
        self.files_affected.push(change);
    }

    /// Set parent execution ID
    pub fn with_parent(mut self, parent_id: ToolExecId) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    /// Set session ID
    pub fn with_session(mut self, session_id: impl Into<String>) -> Self {
        self.session_id = Some(session_id.into());
        self
    }

    /// Set model
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }
}

/// Type of file change
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileChangeType {
    Create,
    Modify,
    Delete,
    Rename,
    Read,
}

/// A single file change record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChange {
    /// Type of change
    pub change_type: FileChangeType,
    /// File path (relative to workspace)
    pub path: String,
    /// Old path (for renames)
    pub old_path: Option<String>,
    /// Lines affected (start, end) - 1-indexed
    pub lines_affected: Option<(u32, u32)>,
    /// Content before change (for modifications)
    pub before: Option<String>,
    /// Content after change (for modifications/creates)
    pub after: Option<String>,
    /// Unified diff (if available)
    pub diff: Option<String>,
    /// Size in bytes before
    pub size_before: Option<u64>,
    /// Size in bytes after
    pub size_after: Option<u64>,
    /// Timestamp (Unix ms)
    pub timestamp: u64,
}

impl FileChange {
    /// Create a file creation record
    pub fn create(path: impl Into<String>, content: impl Into<String>) -> Self {
        let content = content.into();
        let lines = content.lines().count() as u32;
        Self {
            change_type: FileChangeType::Create,
            path: path.into(),
            old_path: None,
            lines_affected: Some((1, lines)),
            before: None,
            after: Some(content.clone()),
            diff: None,
            size_before: None,
            size_after: Some(content.len() as u64),
            timestamp: current_timestamp_ms(),
        }
    }

    /// Create a file modification record
    pub fn modify(
        path: impl Into<String>,
        before: impl Into<String>,
        after: impl Into<String>,
        lines: Option<(u32, u32)>,
    ) -> Self {
        let before = before.into();
        let after = after.into();
        Self {
            change_type: FileChangeType::Modify,
            path: path.into(),
            old_path: None,
            lines_affected: lines,
            before: Some(before.clone()),
            after: Some(after.clone()),
            diff: None,
            size_before: Some(before.len() as u64),
            size_after: Some(after.len() as u64),
            timestamp: current_timestamp_ms(),
        }
    }

    /// Create a file modification with diff
    pub fn modify_with_diff(
        path: impl Into<String>,
        before: impl Into<String>,
        after: impl Into<String>,
        diff: impl Into<String>,
        lines: Option<(u32, u32)>,
    ) -> Self {
        let mut change = Self::modify(path, before, after, lines);
        change.diff = Some(diff.into());
        change
    }

    /// Create a file deletion record
    pub fn delete(path: impl Into<String>, content: impl Into<String>) -> Self {
        let content = content.into();
        Self {
            change_type: FileChangeType::Delete,
            path: path.into(),
            old_path: None,
            lines_affected: None,
            before: Some(content.clone()),
            after: None,
            diff: None,
            size_before: Some(content.len() as u64),
            size_after: None,
            timestamp: current_timestamp_ms(),
        }
    }

    /// Create a file read record (for audit trail)
    pub fn read(path: impl Into<String>, lines: Option<(u32, u32)>) -> Self {
        Self {
            change_type: FileChangeType::Read,
            path: path.into(),
            old_path: None,
            lines_affected: lines,
            before: None,
            after: None,
            diff: None,
            size_before: None,
            size_after: None,
            timestamp: current_timestamp_ms(),
        }
    }

    /// Create a file rename record
    pub fn rename(old_path: impl Into<String>, new_path: impl Into<String>) -> Self {
        Self {
            change_type: FileChangeType::Rename,
            path: new_path.into(),
            old_path: Some(old_path.into()),
            lines_affected: None,
            before: None,
            after: None,
            diff: None,
            size_before: None,
            size_after: None,
            timestamp: current_timestamp_ms(),
        }
    }

    /// Get a short summary of the change
    pub fn summary(&self) -> String {
        match self.change_type {
            FileChangeType::Create => format!("+ {}", self.path),
            FileChangeType::Modify => {
                if let Some((start, end)) = self.lines_affected {
                    format!("M {} (L{}-{})", self.path, start, end)
                } else {
                    format!("M {}", self.path)
                }
            }
            FileChangeType::Delete => format!("- {}", self.path),
            FileChangeType::Rename => format!(
                "R {} -> {}",
                self.old_path.as_deref().unwrap_or("?"),
                self.path
            ),
            FileChangeType::Read => {
                if let Some((start, end)) = self.lines_affected {
                    format!("R {} (L{}-{})", self.path, start, end)
                } else {
                    format!("R {}", self.path)
                }
            }
        }
    }
}

/// Thread-safe tool execution tracker
#[derive(Debug)]
pub struct ToolExecutionTracker {
    /// All executions (limited to last N)
    executions: RwLock<Vec<ToolExecution>>,
    /// Executions by tool name
    by_tool: RwLock<HashMap<String, Vec<ToolExecId>>>,
    /// Executions by file path
    by_file: RwLock<HashMap<String, Vec<ToolExecId>>>,
    /// Max executions to retain
    max_executions: usize,
    /// Total execution count (even if some evicted)
    total_count: AtomicU64,
    /// Total duration across all executions
    total_duration_ms: AtomicU64,
    /// Error count
    error_count: AtomicU64,
}

impl ToolExecutionTracker {
    /// Create a new tracker with default capacity (1000)
    pub fn new() -> Self {
        Self::with_capacity(1000)
    }

    /// Create a tracker with specified capacity
    pub fn with_capacity(max_executions: usize) -> Self {
        Self {
            executions: RwLock::new(Vec::with_capacity(max_executions)),
            by_tool: RwLock::new(HashMap::new()),
            by_file: RwLock::new(HashMap::new()),
            max_executions,
            total_count: AtomicU64::new(0),
            total_duration_ms: AtomicU64::new(0),
            error_count: AtomicU64::new(0),
        }
    }

    /// Record a completed tool execution
    pub fn record(&self, execution: ToolExecution) {
        let exec_id = execution.id;
        let tool_name = execution.tool_name.clone();
        let files: Vec<String> = execution
            .files_affected
            .iter()
            .map(|f| f.path.clone())
            .collect();
        let duration = execution.duration_ms;
        let success = execution.success;

        // Update atomics
        self.total_count.fetch_add(1, Ordering::Relaxed);
        self.total_duration_ms
            .fetch_add(duration, Ordering::Relaxed);
        if !success {
            self.error_count.fetch_add(1, Ordering::Relaxed);
        }

        // Store execution
        {
            let mut execs = self.executions.write();
            if execs.len() >= self.max_executions {
                execs.remove(0);
            }
            execs.push(execution);
        }

        // Index by tool
        {
            let mut by_tool = self.by_tool.write();
            by_tool.entry(tool_name).or_default().push(exec_id);
        }

        // Index by file
        {
            let mut by_file = self.by_file.write();
            for file in files {
                by_file.entry(file).or_default().push(exec_id);
            }
        }
    }

    /// Get execution by ID
    pub fn get(&self, id: ToolExecId) -> Option<ToolExecution> {
        let execs = self.executions.read();
        execs.iter().find(|e| e.id == id).cloned()
    }

    /// Get all executions for a tool
    pub fn get_by_tool(&self, tool_name: &str) -> Vec<ToolExecution> {
        let ids = {
            let by_tool = self.by_tool.read();
            by_tool.get(tool_name).cloned().unwrap_or_default()
        };

        let execs = self.executions.read();
        ids.iter()
            .filter_map(|id| execs.iter().find(|e| e.id == *id).cloned())
            .collect()
    }

    /// Get all executions that affected a file
    pub fn get_by_file(&self, path: &str) -> Vec<ToolExecution> {
        let ids = {
            let by_file = self.by_file.read();
            by_file.get(path).cloned().unwrap_or_default()
        };

        let execs = self.executions.read();
        ids.iter()
            .filter_map(|id| execs.iter().find(|e| e.id == *id).cloned())
            .collect()
    }

    /// Get recent executions (last N)
    pub fn recent(&self, count: usize) -> Vec<ToolExecution> {
        let execs = self.executions.read();
        execs.iter().rev().take(count).cloned().collect()
    }

    /// Get all file changes across all executions
    pub fn all_file_changes(&self) -> Vec<(ToolExecId, FileChange)> {
        let execs = self.executions.read();
        execs
            .iter()
            .flat_map(|e| e.files_affected.iter().map(|f| (e.id, f.clone())))
            .collect()
    }

    /// Get statistics
    pub fn stats(&self) -> ToolExecutionStats {
        let total = self.total_count.load(Ordering::Relaxed);
        let errors = self.error_count.load(Ordering::Relaxed);
        let duration = self.total_duration_ms.load(Ordering::Relaxed);

        let execs = self.executions.read();
        let by_tool = self.by_tool.read();

        let tool_counts: HashMap<String, u64> = by_tool
            .iter()
            .map(|(k, v)| (k.clone(), v.len() as u64))
            .collect();

        let file_count = execs
            .iter()
            .flat_map(|e| e.files_affected.iter().map(|f| f.path.clone()))
            .collect::<std::collections::HashSet<_>>()
            .len();

        ToolExecutionStats {
            total_executions: total,
            successful_executions: total.saturating_sub(errors),
            failed_executions: errors,
            total_duration_ms: duration,
            avg_duration_ms: if total > 0 { duration / total } else { 0 },
            executions_by_tool: tool_counts,
            unique_files_affected: file_count as u64,
        }
    }
}

impl Default for ToolExecutionTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about tool executions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolExecutionStats {
    pub total_executions: u64,
    pub successful_executions: u64,
    pub failed_executions: u64,
    pub total_duration_ms: u64,
    pub avg_duration_ms: u64,
    pub executions_by_tool: HashMap<String, u64>,
    pub unique_files_affected: u64,
}

impl ToolExecutionStats {
    /// Format as summary string
    pub fn summary(&self) -> String {
        let success_rate = if self.total_executions > 0 {
            (self.successful_executions as f64 / self.total_executions as f64) * 100.0
        } else {
            100.0
        };

        format!(
            "{} executions ({:.1}% success), {} files, avg {:.0}ms",
            self.total_executions, success_rate, self.unique_files_affected, self.avg_duration_ms
        )
    }
}

/// Helper to get current timestamp in milliseconds
fn current_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

/// Global tool execution tracker
pub static TOOL_EXECUTIONS: once_cell::sync::Lazy<ToolExecutionTracker> =
    once_cell::sync::Lazy::new(ToolExecutionTracker::new);

// ============================================================================
// Token Usage Tracking (existing code below)
// ============================================================================

/// Token counts for a single request/operation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TokenCounts {
    /// Number of tokens in the input/prompt
    pub input: u64,
    /// Number of tokens in the output/completion
    pub output: u64,
}

impl TokenCounts {
    /// Create new token counts
    pub fn new(input: u64, output: u64) -> Self {
        Self { input, output }
    }

    /// Total tokens (input + output)
    pub fn total(&self) -> u64 {
        self.input.saturating_add(self.output)
    }

    /// Add another TokenCounts to this one
    pub fn add(&mut self, other: &TokenCounts) {
        self.input = self.input.saturating_add(other.input);
        self.output = self.output.saturating_add(other.output);
    }
}

impl std::fmt::Display for TokenCounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} in / {} out ({} total)",
            self.input,
            self.output,
            self.total()
        )
    }
}

/// Statistics for token usage
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct TokenStats {
    /// Average tokens per request
    pub avg_input: f64,
    pub avg_output: f64,
    pub avg_total: f64,
    /// Maximum tokens in a single request
    pub max_input: u64,
    pub max_output: u64,
    pub max_total: u64,
}

/// Thread-safe token usage tracker for a single model or operation type
#[derive(Debug)]
pub struct TokenUsageTracker {
    /// Atomic counters for thread-safe updates
    total_input: AtomicU64,
    total_output: AtomicU64,
    total_tokens: AtomicU64,
    request_count: AtomicU64,
    max_input: AtomicU64,
    max_output: AtomicU64,
    max_total: AtomicU64,
    /// Model or operation name
    name: String,
}

impl TokenUsageTracker {
    /// Create a new tracker for a named model/operation
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            total_input: AtomicU64::new(0),
            total_output: AtomicU64::new(0),
            total_tokens: AtomicU64::new(0),
            request_count: AtomicU64::new(0),
            max_input: AtomicU64::new(0),
            max_output: AtomicU64::new(0),
            max_total: AtomicU64::new(0),
            name: name.into(),
        }
    }

    /// Record token usage for a single request
    pub fn record(&self, input: u64, output: u64) {
        let counts = TokenCounts::new(input, output);

        // Update atomics
        self.total_input.fetch_add(input, Ordering::Relaxed);
        self.total_output.fetch_add(output, Ordering::Relaxed);
        self.total_tokens
            .fetch_add(counts.total(), Ordering::Relaxed);
        let _count = self.request_count.fetch_add(1, Ordering::Relaxed) + 1;

        // Update max values
        self.max_input.fetch_max(input, Ordering::Relaxed);
        self.max_output.fetch_max(output, Ordering::Relaxed);
        self.max_total.fetch_max(counts.total(), Ordering::Relaxed);
    }

    /// Record with TokenCounts struct
    pub fn record_counts(&self, counts: &TokenCounts) {
        self.record(counts.input, counts.output);
    }

    /// Get current totals (fast, atomic)
    pub fn totals(&self) -> TokenCounts {
        TokenCounts {
            input: self.total_input.load(Ordering::Relaxed),
            output: self.total_output.load(Ordering::Relaxed),
        }
    }

    /// Get the request count
    pub fn request_count(&self) -> u64 {
        self.request_count.load(Ordering::Relaxed)
    }

    /// Get a snapshot of current state
    pub fn snapshot(&self) -> TokenUsageSnapshot {
        let totals = self.totals();
        let request_count = self.request_count();
        let max_input = self.max_input.load(Ordering::Relaxed);
        let max_output = self.max_output.load(Ordering::Relaxed);
        let max_total = self.max_total.load(Ordering::Relaxed);

        let stats = TokenStats {
            avg_input: if request_count > 0 {
                totals.input as f64 / request_count as f64
            } else {
                0.0
            },
            avg_output: if request_count > 0 {
                totals.output as f64 / request_count as f64
            } else {
                0.0
            },
            avg_total: if request_count > 0 {
                totals.total() as f64 / request_count as f64
            } else {
                0.0
            },
            max_input,
            max_output,
            max_total,
        };

        TokenUsageSnapshot {
            name: self.name.clone(),
            totals,
            request_count,
            stats,
        }
    }
}

/// Immutable snapshot of token usage state
#[derive(Debug, Clone)]
pub struct TokenUsageSnapshot {
    pub name: String,
    pub totals: TokenCounts,
    pub request_count: u64,
    pub stats: TokenStats,
}

impl TokenUsageSnapshot {
    /// Display summary
    pub fn summary(&self) -> String {
        format!(
            "{}: {} tokens ({} requests)",
            self.name,
            self.totals.total(),
            self.request_count
        )
    }

    /// Display detailed stats
    pub fn detailed(&self) -> String {
        format!(
            "{}: {} tokens ({} requests)\n  Avg: {:.1} in / {:.1} out\n  Max: {} in / {} out",
            self.name,
            self.totals.total(),
            self.request_count,
            self.stats.avg_input,
            self.stats.avg_output,
            self.stats.max_input,
            self.stats.max_output
        )
    }
}

/// Tracks usage by model, by operation type, and provides aggregation
#[derive(Debug)]
pub struct TokenUsageRegistry {
    /// Trackers by model name
    by_model: RwLock<HashMap<String, Arc<TokenUsageTracker>>>,
    /// Trackers by operation type
    by_operation: RwLock<HashMap<String, Arc<TokenUsageTracker>>>,
    /// Global tracker for all usage
    global: Arc<TokenUsageTracker>,
}

impl TokenUsageRegistry {
    /// Create new registry
    pub fn new() -> Self {
        Self {
            by_model: RwLock::new(HashMap::new()),
            by_operation: RwLock::new(HashMap::new()),
            global: Arc::new(TokenUsageTracker::new("global")),
        }
    }

    /// Record usage for a specific model
    pub fn record_model_usage(&self, model: &str, input: u64, output: u64) {
        // Get or create tracker for this model
        let tracker = {
            let mut models = self.by_model.write();
            models
                .entry(model.to_string())
                .or_insert_with(|| Arc::new(TokenUsageTracker::new(model)))
                .clone()
        };

        tracker.record(input, output);
        self.global.record(input, output);
    }

    /// Record usage for a specific operation type
    pub fn record_operation_usage(&self, operation: &str, input: u64, output: u64) {
        let tracker = {
            let mut operations = self.by_operation.write();
            operations
                .entry(operation.to_string())
                .or_insert_with(|| Arc::new(TokenUsageTracker::new(operation)))
                .clone()
        };

        tracker.record(input, output);
    }

    /// Get tracker for a specific model
    pub fn get_model_tracker(&self, model: &str) -> Option<Arc<TokenUsageTracker>> {
        let models = self.by_model.read();
        models.get(model).cloned()
    }

    /// Get tracker for a specific operation
    pub fn get_operation_tracker(&self, operation: &str) -> Option<Arc<TokenUsageTracker>> {
        let operations = self.by_operation.read();
        operations.get(operation).cloned()
    }

    /// Get global tracker
    pub fn global_tracker(&self) -> Arc<TokenUsageTracker> {
        self.global.clone()
    }

    /// Get all model snapshots
    pub fn model_snapshots(&self) -> Vec<TokenUsageSnapshot> {
        let models = self.by_model.read();
        models.values().map(|tracker| tracker.snapshot()).collect()
    }

    /// Get all operation snapshots
    pub fn operation_snapshots(&self) -> Vec<TokenUsageSnapshot> {
        let operations = self.by_operation.read();
        operations
            .values()
            .map(|tracker| tracker.snapshot())
            .collect()
    }

    /// Get global snapshot
    pub fn global_snapshot(&self) -> TokenUsageSnapshot {
        self.global.snapshot()
    }
}

impl Default for TokenUsageRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Cost calculation utilities
#[derive(Debug, Clone, Copy, Default)]
pub struct CostEstimate {
    pub input_cost: f64,
    pub output_cost: f64,
    pub total_cost: f64,
}

impl CostEstimate {
    /// Calculate cost from token counts and pricing
    pub fn from_tokens(
        counts: &TokenCounts,
        input_cost_per_million: f64,
        output_cost_per_million: f64,
    ) -> Self {
        let input_cost = (counts.input as f64 / 1_000_000.0) * input_cost_per_million;
        let output_cost = (counts.output as f64 / 1_000_000.0) * output_cost_per_million;
        let total_cost = input_cost + output_cost;

        Self {
            input_cost,
            output_cost,
            total_cost,
        }
    }

    /// Format cost as currency
    pub fn format_currency(&self) -> String {
        format!("${:.4}", self.total_cost)
    }

    /// Format cost with appropriate precision
    pub fn format_smart(&self) -> String {
        if self.total_cost < 0.0001 {
            "< $0.0001".to_string()
        } else if self.total_cost < 0.01 {
            format!("${:.4}", self.total_cost)
        } else if self.total_cost < 1.0 {
            format!("${:.3}", self.total_cost)
        } else {
            format!("${:.2}", self.total_cost)
        }
    }
}

/// Context limit tracking
#[derive(Debug, Clone, Copy, Default)]
pub struct ContextLimit {
    pub current: u64,
    pub limit: u64,
    pub percentage: f64,
}

impl ContextLimit {
    /// Create new context limit info
    pub fn new(current: u64, limit: u64) -> Self {
        let percentage = if limit > 0 {
            (current as f64 / limit as f64) * 100.0
        } else {
            0.0
        };
        Self {
            current,
            limit,
            percentage,
        }
    }

    /// Get warning level based on percentage
    pub fn warning_level(&self) -> &'static str {
        match self.percentage {
            p if p >= 100.0 => "CRITICAL",
            p if p >= 90.0 => "HIGH",
            p if p >= 75.0 => "MEDIUM",
            p if p >= 50.0 => "LOW",
            _ => "OK",
        }
    }

    /// Get color for warning level
    pub fn warning_color(&self) -> Color {
        match self.warning_level() {
            "CRITICAL" => Color::Red,
            "HIGH" => Color::LightRed,
            "MEDIUM" => Color::Yellow,
            "LOW" => Color::LightYellow,
            _ => Color::Green,
        }
    }
}

/// Global token usage registry
pub static TOKEN_USAGE: once_cell::sync::Lazy<TokenUsageRegistry> =
    once_cell::sync::Lazy::new(TokenUsageRegistry::new);

// ============================================================================
// Telemetry Persistence
// ============================================================================

/// Persistent telemetry data that can be saved/loaded from disk
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TelemetryData {
    /// Tool executions (last N)
    pub executions: Vec<ToolExecution>,
    /// Summary stats
    pub stats: TelemetryStats,
    /// Last updated timestamp
    pub last_updated: u64,
}

/// Aggregated statistics for persistence
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TelemetryStats {
    pub total_executions: u64,
    pub successful_executions: u64,
    pub failed_executions: u64,
    pub total_duration_ms: u64,
    pub total_input_tokens: u64,
    pub total_output_tokens: u64,
    pub total_requests: u64,
    pub executions_by_tool: HashMap<String, u64>,
    pub files_modified: HashMap<String, u64>,
}

impl TelemetryData {
    /// Get the default telemetry file path
    pub fn default_path() -> std::path::PathBuf {
        directories::ProjectDirs::from("com", "codetether", "codetether")
            .map(|p| p.data_dir().join("telemetry.json"))
            .unwrap_or_else(|| std::path::PathBuf::from(".codetether/telemetry.json"))
    }

    /// Load telemetry from disk
    pub fn load() -> Self {
        Self::load_from(&Self::default_path())
    }

    /// Load telemetry from a specific path
    pub fn load_from(path: &std::path::Path) -> Self {
        match std::fs::read_to_string(path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    /// Save telemetry to disk
    pub fn save(&self) -> std::io::Result<()> {
        self.save_to(&Self::default_path())
    }

    /// Save telemetry to a specific path
    pub fn save_to(&self, path: &std::path::Path) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)
    }

    /// Add an execution and update stats
    pub fn record_execution(&mut self, exec: ToolExecution) {
        // Update stats
        self.stats.total_executions += 1;
        if exec.success {
            self.stats.successful_executions += 1;
        } else {
            self.stats.failed_executions += 1;
        }
        self.stats.total_duration_ms += exec.duration_ms;

        // Track by tool
        *self
            .stats
            .executions_by_tool
            .entry(exec.tool_name.clone())
            .or_insert(0) += 1;

        // Track files modified
        for file in &exec.files_affected {
            if file.change_type != FileChangeType::Read {
                *self
                    .stats
                    .files_modified
                    .entry(file.path.clone())
                    .or_insert(0) += 1;
            }
        }

        // Track tokens if present
        if let Some(tokens) = &exec.tokens {
            self.stats.total_input_tokens += tokens.input;
            self.stats.total_output_tokens += tokens.output;
            self.stats.total_requests += 1;
        }

        // Keep only recent executions (limit to 500)
        if self.executions.len() >= 500 {
            self.executions.remove(0);
        }
        self.executions.push(exec);

        self.last_updated = current_timestamp_ms();
    }

    /// Get recent executions
    pub fn recent(&self, count: usize) -> Vec<&ToolExecution> {
        self.executions.iter().rev().take(count).collect()
    }

    /// Get executions by tool name
    pub fn by_tool(&self, tool_name: &str) -> Vec<&ToolExecution> {
        self.executions
            .iter()
            .filter(|e| e.tool_name == tool_name)
            .collect()
    }

    /// Get executions that affected a file
    pub fn by_file(&self, path: &str) -> Vec<&ToolExecution> {
        self.executions
            .iter()
            .filter(|e| e.files_affected.iter().any(|f| f.path == path))
            .collect()
    }

    /// Get all file changes
    pub fn all_file_changes(&self) -> Vec<(ToolExecId, &FileChange)> {
        self.executions
            .iter()
            .flat_map(|e| e.files_affected.iter().map(move |f| (e.id, f)))
            .collect()
    }

    /// Format as summary
    pub fn summary(&self) -> String {
        let success_rate = if self.stats.total_executions > 0 {
            (self.stats.successful_executions as f64 / self.stats.total_executions as f64) * 100.0
        } else {
            100.0
        };

        format!(
            "{} executions ({:.1}% success), {} files modified, {} tokens used",
            self.stats.total_executions,
            success_rate,
            self.stats.files_modified.len(),
            self.stats.total_input_tokens + self.stats.total_output_tokens
        )
    }
}

/// Global persistent telemetry (loaded on first access, saved on record)
pub static PERSISTENT_TELEMETRY: once_cell::sync::Lazy<RwLock<TelemetryData>> =
    once_cell::sync::Lazy::new(|| RwLock::new(TelemetryData::load()));

/// Record an execution to persistent telemetry
pub fn record_persistent(exec: ToolExecution) {
    let mut data = PERSISTENT_TELEMETRY.write();
    data.record_execution(exec);
    // Best effort save - don't fail the operation if save fails
    let _ = data.save();
}

/// Get a snapshot of persistent telemetry
pub fn get_persistent_stats() -> TelemetryData {
    PERSISTENT_TELEMETRY.read().clone()
}

// ============================================================================
// Swarm Telemetry (for executor.rs)
// ============================================================================

/// Unique identifier for a swarm execution
pub type SwarmExecId = String;

/// Unique identifier for a sub-agent execution
pub type SubAgentExecId = u64;

/// Privacy level for telemetry data
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PrivacyLevel {
    /// Full telemetry (all data)
    Full,
    /// Reduced telemetry (metadata only, no content)
    #[default]
    Reduced,
    /// Minimal telemetry (counts only)
    Minimal,
    /// No telemetry
    None,
}

/// Metrics collected during swarm execution
#[derive(Debug, Clone, Default)]
pub struct SwarmTelemetryMetrics {
    /// Swarm execution ID
    pub swarm_id: SwarmExecId,
    /// Number of subtasks
    pub subtask_count: usize,
    /// Execution strategy used
    pub strategy: String,
    /// Whether execution succeeded
    pub success: bool,
    /// Total execution time
    pub total_duration_ms: u64,
    /// Stage-level metrics
    pub stage_metrics: Vec<StageTelemetry>,
}

/// Per-stage telemetry
#[derive(Debug, Clone, Default)]
pub struct StageTelemetry {
    /// Stage number
    pub stage: usize,
    /// Number of subagents in stage
    pub subagent_count: usize,
    /// Successful completions
    pub completed: usize,
    /// Failures
    pub failed: usize,
    /// Stage duration
    pub duration_ms: u64,
}

/// Telemetry collector for swarm operations
#[derive(Debug, Default)]
pub struct SwarmTelemetryCollector {
    /// Current swarm ID
    swarm_id: Option<SwarmExecId>,
    /// Start time
    start_time: Option<std::time::Instant>,
    /// Collected metrics
    metrics: Option<SwarmTelemetryMetrics>,
}

impl SwarmTelemetryCollector {
    /// Create a new telemetry collector
    pub fn new() -> Self {
        Self::default()
    }

    /// Start tracking a swarm execution
    pub fn start_swarm(&mut self, swarm_id: SwarmExecId, subtask_count: usize, strategy: &str) {
        self.swarm_id = Some(swarm_id.clone());
        self.start_time = Some(std::time::Instant::now());
        self.metrics = Some(SwarmTelemetryMetrics {
            swarm_id,
            subtask_count,
            strategy: strategy.to_string(),
            ..Default::default()
        });
    }

    /// Record latency for a specific operation
    pub fn record_swarm_latency(&self, _operation: &str, _duration: std::time::Duration) {
        // Placeholder for latency tracking
    }

    /// Complete swarm tracking and return metrics
    pub fn complete_swarm(&mut self, success: bool) -> SwarmTelemetryMetrics {
        let mut metrics = self.metrics.take().unwrap_or_default();
        metrics.success = success;
        if let Some(start) = self.start_time {
            metrics.total_duration_ms = start.elapsed().as_millis() as u64;
        }
        metrics
    }
}

// ============================================================================
// Provider Performance Metrics (latency, throughput, TPS)
// ============================================================================

/// A single recorded provider request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderRequestRecord {
    /// Provider name (e.g. "openrouter", "anthropic")
    pub provider: String,
    /// Model used
    pub model: String,
    /// Total latency in milliseconds (request to response)
    pub latency_ms: u64,
    /// Time to first token in milliseconds (for streaming, 0 for non-streaming)
    pub ttft_ms: u64,
    /// Input tokens
    pub input_tokens: u64,
    /// Output tokens
    pub output_tokens: u64,
    /// Whether the request succeeded
    pub success: bool,
    /// Timestamp (Unix ms)
    pub timestamp: u64,
}

impl ProviderRequestRecord {
    /// Tokens per second (output tokens / latency)
    pub fn tokens_per_second(&self) -> f64 {
        if self.latency_ms == 0 {
            return 0.0;
        }
        (self.output_tokens as f64) / (self.latency_ms as f64 / 1000.0)
    }
}

/// Per-provider performance tracker using atomics for fast concurrent updates
#[derive(Debug)]
pub struct ProviderPerformanceTracker {
    /// Provider name
    name: String,
    /// Total request count
    request_count: AtomicU64,
    /// Total successful requests
    success_count: AtomicU64,
    /// Total failed requests
    error_count: AtomicU64,
    /// Sum of all latencies (ms) for average calculation
    total_latency_ms: AtomicU64,
    /// Minimum latency (ms)
    min_latency_ms: AtomicU64,
    /// Maximum latency (ms)
    max_latency_ms: AtomicU64,
    /// Sum of all TTFT values (ms)
    total_ttft_ms: AtomicU64,
    /// Total output tokens (for TPS calculation)
    total_output_tokens: AtomicU64,
    /// Total input tokens
    total_input_tokens: AtomicU64,
    /// Recent request records (for percentile calculations)
    recent_records: RwLock<Vec<ProviderRequestRecord>>,
    /// Max recent records to retain
    max_recent: usize,
}

impl ProviderPerformanceTracker {
    /// Create a new tracker for a named provider
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            request_count: AtomicU64::new(0),
            success_count: AtomicU64::new(0),
            error_count: AtomicU64::new(0),
            total_latency_ms: AtomicU64::new(0),
            min_latency_ms: AtomicU64::new(u64::MAX),
            max_latency_ms: AtomicU64::new(0),
            total_ttft_ms: AtomicU64::new(0),
            total_output_tokens: AtomicU64::new(0),
            total_input_tokens: AtomicU64::new(0),
            recent_records: RwLock::new(Vec::with_capacity(200)),
            max_recent: 200,
        }
    }

    /// Record a completed provider request
    pub fn record(&self, record: ProviderRequestRecord) {
        self.request_count.fetch_add(1, Ordering::Relaxed);
        self.total_latency_ms
            .fetch_add(record.latency_ms, Ordering::Relaxed);
        self.total_output_tokens
            .fetch_add(record.output_tokens, Ordering::Relaxed);
        self.total_input_tokens
            .fetch_add(record.input_tokens, Ordering::Relaxed);
        self.total_ttft_ms
            .fetch_add(record.ttft_ms, Ordering::Relaxed);

        if record.success {
            self.success_count.fetch_add(1, Ordering::Relaxed);
        } else {
            self.error_count.fetch_add(1, Ordering::Relaxed);
        }

        // Update min/max latency
        self.min_latency_ms
            .fetch_min(record.latency_ms, Ordering::Relaxed);
        self.max_latency_ms
            .fetch_max(record.latency_ms, Ordering::Relaxed);

        // Store recent record
        let mut records = self.recent_records.write();
        if records.len() >= self.max_recent {
            records.remove(0);
        }
        records.push(record);
    }

    /// Get a performance snapshot
    pub fn snapshot(&self) -> ProviderPerformanceSnapshot {
        let request_count = self.request_count.load(Ordering::Relaxed);
        let total_latency = self.total_latency_ms.load(Ordering::Relaxed);
        let total_output = self.total_output_tokens.load(Ordering::Relaxed);
        let total_input = self.total_input_tokens.load(Ordering::Relaxed);
        let total_ttft = self.total_ttft_ms.load(Ordering::Relaxed);
        let min_latency = self.min_latency_ms.load(Ordering::Relaxed);
        let max_latency = self.max_latency_ms.load(Ordering::Relaxed);

        let avg_latency_ms = if request_count > 0 {
            total_latency as f64 / request_count as f64
        } else {
            0.0
        };

        let avg_tps = if total_latency > 0 {
            (total_output as f64) / (total_latency as f64 / 1000.0)
        } else {
            0.0
        };

        let avg_ttft_ms = if request_count > 0 {
            total_ttft as f64 / request_count as f64
        } else {
            0.0
        };

        // Calculate p50 and p95 from recent records
        let (p50_latency_ms, p95_latency_ms, p50_tps, p95_tps) = {
            let records = self.recent_records.read();
            if records.is_empty() {
                (0.0, 0.0, 0.0, 0.0)
            } else {
                let mut latencies: Vec<u64> =
                    records.iter().map(|r| r.latency_ms).collect();
                latencies.sort_unstable();
                let p50_idx = (latencies.len() as f64 * 0.50) as usize;
                let p95_idx = (latencies.len() as f64 * 0.95).min((latencies.len() - 1) as f64)
                    as usize;
                let p50_lat = latencies[p50_idx] as f64;
                let p95_lat = latencies[p95_idx] as f64;

                let mut tps_values: Vec<f64> = records
                    .iter()
                    .filter(|r| r.latency_ms > 0)
                    .map(|r| r.tokens_per_second())
                    .collect();
                tps_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                let (p50_tps, p95_tps) = if tps_values.is_empty() {
                    (0.0, 0.0)
                } else {
                    let p50_i = (tps_values.len() as f64 * 0.50) as usize;
                    let p95_i = (tps_values.len() as f64 * 0.95)
                        .min((tps_values.len() - 1) as f64)
                        as usize;
                    (tps_values[p50_i], tps_values[p95_i])
                };

                (p50_lat, p95_lat, p50_tps, p95_tps)
            }
        };

        ProviderPerformanceSnapshot {
            provider: self.name.clone(),
            request_count,
            success_count: self.success_count.load(Ordering::Relaxed),
            error_count: self.error_count.load(Ordering::Relaxed),
            avg_latency_ms,
            min_latency_ms: if min_latency == u64::MAX {
                0
            } else {
                min_latency
            },
            max_latency_ms: max_latency,
            p50_latency_ms,
            p95_latency_ms,
            avg_ttft_ms,
            avg_tps,
            p50_tps,
            p95_tps,
            total_input_tokens: total_input,
            total_output_tokens: total_output,
        }
    }
}

/// Immutable snapshot of provider performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderPerformanceSnapshot {
    pub provider: String,
    pub request_count: u64,
    pub success_count: u64,
    pub error_count: u64,
    /// Average latency in ms
    pub avg_latency_ms: f64,
    /// Minimum observed latency in ms
    pub min_latency_ms: u64,
    /// Maximum observed latency in ms
    pub max_latency_ms: u64,
    /// p50 (median) latency in ms
    pub p50_latency_ms: f64,
    /// p95 latency in ms
    pub p95_latency_ms: f64,
    /// Average time to first token (ms)
    pub avg_ttft_ms: f64,
    /// Average tokens per second (output_tokens / seconds)
    pub avg_tps: f64,
    /// p50 tokens per second
    pub p50_tps: f64,
    /// p95 tokens per second
    pub p95_tps: f64,
    /// Total input tokens processed
    pub total_input_tokens: u64,
    /// Total output tokens generated
    pub total_output_tokens: u64,
}

impl ProviderPerformanceSnapshot {
    /// Format as a compact summary line
    pub fn summary(&self) -> String {
        format!(
            "{}: {} reqs, avg {:.0}ms (p50 {:.0}ms, p95 {:.0}ms), {:.1} tok/s, {} errors",
            self.provider,
            self.request_count,
            self.avg_latency_ms,
            self.p50_latency_ms,
            self.p95_latency_ms,
            self.avg_tps,
            self.error_count,
        )
    }

    /// Format as detailed multi-line report
    pub fn detailed(&self) -> String {
        let success_rate = if self.request_count > 0 {
            (self.success_count as f64 / self.request_count as f64) * 100.0
        } else {
            100.0
        };

        format!(
            "{provider}:\n  Requests: {reqs} ({rate:.1}% success)\n  Latency:  avg {avg:.0}ms | p50 {p50:.0}ms | p95 {p95:.0}ms | min {min}ms | max {max}ms\n  TTFT:     avg {ttft:.0}ms\n  TPS:      avg {tps:.1} | p50 {tps50:.1} | p95 {tps95:.1}\n  Tokens:   {input} in / {output} out",
            provider = self.provider,
            reqs = self.request_count,
            rate = success_rate,
            avg = self.avg_latency_ms,
            p50 = self.p50_latency_ms,
            p95 = self.p95_latency_ms,
            min = self.min_latency_ms,
            max = self.max_latency_ms,
            ttft = self.avg_ttft_ms,
            tps = self.avg_tps,
            tps50 = self.p50_tps,
            tps95 = self.p95_tps,
            input = self.total_input_tokens,
            output = self.total_output_tokens,
        )
    }
}

/// Registry of per-provider performance trackers
#[derive(Debug)]
pub struct ProviderMetricsRegistry {
    trackers: RwLock<HashMap<String, Arc<ProviderPerformanceTracker>>>,
}

impl ProviderMetricsRegistry {
    pub fn new() -> Self {
        Self {
            trackers: RwLock::new(HashMap::new()),
        }
    }

    /// Record a provider request
    pub fn record(&self, record: ProviderRequestRecord) {
        let provider = record.provider.clone();
        let tracker = {
            let mut trackers = self.trackers.write();
            trackers
                .entry(provider.clone())
                .or_insert_with(|| Arc::new(ProviderPerformanceTracker::new(provider)))
                .clone()
        };
        tracker.record(record);
    }

    /// Get snapshot for a specific provider
    pub fn provider_snapshot(&self, provider: &str) -> Option<ProviderPerformanceSnapshot> {
        let trackers = self.trackers.read();
        trackers.get(provider).map(|t| t.snapshot())
    }

    /// Get snapshots for all providers
    pub fn all_snapshots(&self) -> Vec<ProviderPerformanceSnapshot> {
        let trackers = self.trackers.read();
        trackers.values().map(|t| t.snapshot()).collect()
    }

    /// Get provider names
    pub fn providers(&self) -> Vec<String> {
        let trackers = self.trackers.read();
        trackers.keys().cloned().collect()
    }
}

impl Default for ProviderMetricsRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Global provider metrics registry
pub static PROVIDER_METRICS: once_cell::sync::Lazy<ProviderMetricsRegistry> =
    once_cell::sync::Lazy::new(ProviderMetricsRegistry::new);
