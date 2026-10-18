//! Replay-oracle evaluation harness (ClawVM §3).
//!
//! ## Role
//!
//! ClawVM proposes a replay oracle to separate *policy quality* from
//! *budget insufficiency*: given a recorded trace and the same budget,
//! an oracle with bounded future lookahead `h` picks representations
//! that minimise faults. The online-minus-oracle fault gap then
//! measures headroom vs. unavoidable workload pressure.
//!
//! ## Scope (Phase B step 22)
//!
//! This module delivers the evaluation primitive without wiring it
//! into the live prompt loop (it is intentionally offline). It takes
//! a recorded `Vec<Message>` plus a horizon `h` and produces a
//! per-turn demand trace — which prior messages get referenced within
//! the next `h` turns. That trace is the "ground truth" the Pareto
//! eval harness can grade any [`codetether_context_policy::derivation::DerivePolicy`] against.
//!
//! Demand signal today is a conservative lexical heuristic: a turn at
//! index `t` is said to "need" turn `u` (with `u < t`) when:
//!
//! * The body of `t` mentions a file path that first appeared in `u`.
//! * A `ToolResult` at `t` matches a `ToolCall` id introduced at `u`.
//!
//! Both signals are overly conservative — the real oracle can see
//! model-internal attention — but they are faithful enough to seed
//! the oracle-gap metric and to motivate future refinement.
//!
//! ## Invariants
//!
//! * Pure and offline: no provider, RLM, or filesystem IO.
//! * Deterministic: same input → same output.
//! * The full message vector is borrowed read-only; no mutation.
//!
//! ## Examples
//!
//! ```rust
//! use codetether_provider_core::{ContentPart, Message, Role};
//! use codetether_replay_oracle::{replay_oracle, OracleReport};
//!
//! let msgs = vec![
//!     Message {
//!         role: Role::User,
//!         content: vec![ContentPart::Text { text: "edit src/lib.rs".into() }],
//!     },
//!     Message {
//!         role: Role::Assistant,
//!         content: vec![ContentPart::ToolCall {
//!             id: "call-1".into(),
//!             name: "Shell".into(),
//!             arguments: "{}".into(),
//!             thought_signature: None,
//!         }],
//!     },
//!     Message {
//!         role: Role::Tool,
//!         content: vec![ContentPart::ToolResult {
//!             tool_call_id: "call-1".into(),
//!             content: "ok".into(),
//!         }],
//!     },
//! ];
//! let report: OracleReport = replay_oracle(&msgs, 2);
//! assert_eq!(report.demand.len(), msgs.len());
//! ```

mod index;
mod references;
mod replay;
mod report;
mod tokens;
pub use replay::replay_oracle;
pub use report::OracleReport;
#[cfg(test)]
mod tests;
/// Trace-directory evaluation entry point, currently a placeholder.
pub mod trace;
