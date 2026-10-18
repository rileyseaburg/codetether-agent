//! # Telemetry
//!
//! Observability for the CodeTether agent. Tracks token usage, tool executions,
//! per-provider latency/throughput, file changes, swarm progress, cost
//! estimates, and persistent audit-style records.
//!
//! ## Architecture
//!
//! Every telemetry concern lives in its own submodule so that individual files
//! stay small (≤ 50 lines of code, per the project's [coding standards]) and
//! each file holds a **single responsibility**. This module re-exports the
//! public surface so callers continue to write `use crate::Foo;`
//! without knowing the internal layout.
//!
//! | Submodule | Responsibility |
//! |---|---|
//! | [`tokens`]   | Token totals, snapshots, and the global [`AtomicTokenCounter`] |
//! | [`tools`]    | Tool-execution records, the [`AtomicToolCounter`], and [`FileChange`] |
//! | [`provider`] | Per-provider request records, snapshots, and aggregated stats |
//! | [`context`]  | Context-window usage ratios ([`ContextLimit`]) |
//! | [`cost`]     | USD cost estimation ([`CostEstimate`]) |
//! | [`a2a`]      | A2A message records |
//! | [`swarm`]    | Swarm (multi-agent) telemetry collector |
//! | [`metrics`]  | Per-instance rolling metrics ([`Telemetry`]) |
//! | [`persistent`] | On-disk / long-lived stats façade |
//! | [`globals`]  | Process-wide `Lazy` singletons |
//!
//! ## Coding Standards for This Module
//!
//! New code must follow these rules (enforced in review):
//!
//! 1. **Single Responsibility.** One struct or one tight function group per file.
//! 2. **50-line limit.** Hard cap per file (excluding comments/blanks). Split
//!    before you reach 45.
//! 3. **Rustdoc everywhere.** Every public item gets `///` docs with a
//!    `# Examples` section. Prefer runnable (` ```rust `) examples;
//!    fall back to `no_run` only when a runtime is required; avoid `ignore`.
//! 4. **No `any`-style typing.** Every public field and return type is
//!    explicitly typed.
//! 5. **Tracing over println.** Use structured fields, e.g.
//!    `tracing::info!(tool = %name, "...")`.
//! 6. **Never hand-roll atomics when an existing counter fits.** Reuse
//!    [`AtomicTokenCounter`] / [`AtomicToolCounter`] rather than adding new
//!    `AtomicU64` fields elsewhere.
//!
//! [coding standards]: ../../AGENTS.md
//!
//! ## Quick Start
//!
//! ```rust
//! use codetether_telemetry::{TokenTotals, CostEstimate, TokenCounts};
//!
//! let totals = TokenTotals::new(1_000, 500);
//! assert_eq!(totals.total(), 1_500);
//!
//! let cost = CostEstimate::from_tokens(
//!     &TokenCounts::new(1_000_000, 500_000),
//!     3.00,   // $ per 1M input tokens
//!     15.00,  // $ per 1M output tokens
//! );
//! assert_eq!(cost.currency, "USD");
//! ```

pub mod a2a;
pub mod alloc_guard;
pub(crate) mod alloc_guard_command;
pub mod alloc_guard_config;
pub(crate) mod alloc_guard_payload;
pub mod alloc_guard_report;
#[cfg(test)]
mod alloc_guard_tests;
pub mod context;
pub mod cost;
pub mod crash_context;
#[cfg(test)]
mod crash_context_tests;
pub mod globals;
pub mod memory;
pub mod metrics;
pub mod persistent;
pub mod provider;
pub mod rss_watchdog;
pub mod swarm;
pub(crate) mod system_allocator;
pub mod tokens;
pub mod tools;
pub mod transport;

pub use a2a::A2AMessageRecord;
pub use context::ContextLimit;
pub use cost::CostEstimate;
pub use globals::{PROVIDER_METRICS, TOKEN_USAGE, TOOL_EXECUTIONS, TRANSPORT_METRICS};
pub use metrics::{Telemetry, TelemetryMetrics};
pub use persistent::{
    PersistentStats, PersistentStatsInner, get_persistent_stats, record_persistent,
};
pub use provider::{ProviderMetrics, ProviderRequestRecord, ProviderSnapshot};
pub use swarm::SwarmTelemetryCollector;
pub use tokens::{
    AtomicTokenCounter, GlobalTokenSnapshot, TokenCounts, TokenTotals, TokenUsageSnapshot,
};
pub use tools::{AtomicToolCounter, FileChange, ToolExecution};

/// Arm both memory guards using `report_dir` as the crash-report spool:
/// the capacity-guarding global allocator and the RSS watchdog. Both are
/// idempotent and write into the same spool the flusher drains on startup.
pub fn install_memory_guards(report_dir: std::path::PathBuf) {
    alloc_guard_config::configure(report_dir.clone());
    rss_watchdog::spawn(report_dir);
}

mod application;
pub use application::{application_version, set_application_version};
pub mod tcp_info;

#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATOR: alloc_guard::GuardAlloc = alloc_guard::GuardAlloc;

/// Best-effort recording of completed A2A message exchanges.
pub mod a2a_recording;

/// Bounded rolling timing statistics for completed chat turns.
pub mod chat_latency;

mod request_timing_finish;
mod request_timing_start;
mod request_timing_state;
mod request_timing_tokens;
mod tool_timing_events;
mod tool_timing_state;
pub use request_timing_state::RequestTimingState;
pub use tool_timing_state::ToolTimingState;

#[cfg(test)]
mod request_timing_tests;

/// Application-local UI state adapters.
pub mod state_binding;
