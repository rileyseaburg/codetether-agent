//! # Global telemetry singletons
//!
//! Process-wide counters that every part of the agent writes to. Isolated in
//! their own file so the static definitions don't bloat any type module.
//!
//! ## Why globals?
//!
//! Telemetry must be cheap and lock-free at the call site — wrapping every
//! provider/tool call in dependency-injected collectors was measured to add
//! noticeable overhead in the hot path. The three singletons here use
//! [`AtomicU64`](std::sync::atomic::AtomicU64) internally (for the counters)
//! or a [`tokio::sync::Mutex`] guarded `Vec` (for provider history).
//!
//! ## Examples
//!
//! ```rust
//! use codetether_agent::telemetry::{TOKEN_USAGE, TOOL_EXECUTIONS};
//!
//! TOKEN_USAGE.record(100, 50);
//! TOOL_EXECUTIONS.record(true);
//!
//! let (prompt, completion, total) = TOKEN_USAGE.get();
//! assert!(total >= prompt + completion);
//! ```

use once_cell::sync::Lazy;
use std::sync::Arc;

use super::provider::ProviderMetrics;
use super::tokens::AtomicTokenCounter;
use super::tools::AtomicToolCounter;

/// Process-wide cumulative token usage across every provider request.
pub static TOKEN_USAGE: Lazy<Arc<AtomicTokenCounter>> =
    Lazy::new(|| Arc::new(AtomicTokenCounter::new()));

/// Process-wide cumulative tool execution counter (success + failure).
pub static TOOL_EXECUTIONS: Lazy<Arc<AtomicToolCounter>> =
    Lazy::new(|| Arc::new(AtomicToolCounter::new()));

/// Process-wide rolling buffer of the last N provider requests (N = 1000).
///
/// Used to derive averages, p50, and p95 latency / throughput per provider.
pub static PROVIDER_METRICS: Lazy<Arc<ProviderMetrics>> =
    Lazy::new(|| Arc::new(ProviderMetrics::new()));
