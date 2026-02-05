//! Telemetry and usage tracking
//!
//! Tracks token usage, costs, and other metrics for monitoring agent performance

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use ratatui::style::Color;

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
        let count = self.request_count.fetch_add(1, Ordering::Relaxed) + 1;

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
