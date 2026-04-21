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
//! eval harness can grade any [`DerivePolicy`] against.
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
//! use codetether_agent::provider::{ContentPart, Message, Role};
//! use codetether_agent::session::oracle::{replay_oracle, OracleReport};
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

use std::collections::HashMap;

use crate::provider::{ContentPart, Message};
use crate::session::relevance::extract;

/// Summary of an oracle replay over a recorded trace.
#[derive(Debug, Clone)]
pub struct OracleReport {
    /// For each turn `t` in the trace, the sorted list of prior turn
    /// indices `u < t` that `t` references within the `h`-turn
    /// lookahead window.
    pub demand: Vec<Vec<usize>>,
    /// Horizon the oracle used.
    pub horizon: usize,
}

impl OracleReport {
    /// Number of distinct (t, u) reference edges the oracle observed.
    ///
    /// Useful as a sanity check: zero edges on a long trace usually
    /// means the heuristic wasn't broad enough, not that the agent was
    /// uninteresting.
    pub fn reference_count(&self) -> usize {
        self.demand.iter().map(Vec::len).sum()
    }
}

/// Compute an [`OracleReport`] over `messages` with horizon `h`.
///
/// Given the demand signal (lexical file references plus tool-call id
/// matches) the report records, for every turn `t`, the set of prior
/// turns needed to satisfy demand from `[t, t+h]`. Pareto harnesses
/// compare that set to what a [`DerivePolicy`] actually kept.
pub fn replay_oracle(messages: &[Message], h: usize) -> OracleReport {
    let files_per_turn = index_files_per_turn(messages);
    let tool_call_owner = index_tool_call_owners(messages);

    let mut demand: Vec<Vec<usize>> = vec![Vec::new(); messages.len()];

    for t in 0..messages.len() {
        let window_end = (t + h).min(messages.len().saturating_sub(1));
        for future_t in t..=window_end {
            let future = &messages[future_t];
            for u in references_for_turn(future, t, &files_per_turn, &tool_call_owner) {
                if u < t && !demand[t].contains(&u) {
                    demand[t].push(u);
                }
            }
        }
        demand[t].sort_unstable();
    }

    OracleReport { demand, horizon: h }
}

/// Index files referenced by each turn so later turns can fast-lookup
/// "first turn this path appeared in".
fn index_files_per_turn(messages: &[Message]) -> Vec<Vec<String>> {
    messages.iter().map(|m| extract(m).files).collect()
}

/// Map tool_call_id → owning turn index, so a later `ToolResult` can
/// locate the `ToolCall` that introduced it.
fn index_tool_call_owners(messages: &[Message]) -> HashMap<String, usize> {
    let mut owners = HashMap::new();
    for (idx, msg) in messages.iter().enumerate() {
        for part in &msg.content {
            if let ContentPart::ToolCall { id, .. } = part {
                owners.insert(id.clone(), idx);
            }
        }
    }
    owners
}

/// Compute which prior turns the `future` message references.
fn references_for_turn(
    future: &Message,
    current_idx: usize,
    files_per_turn: &[Vec<String>],
    tool_call_owner: &HashMap<String, usize>,
) -> Vec<usize> {
    let mut out = Vec::new();
    for part in &future.content {
        match part {
            ContentPart::ToolResult { tool_call_id, .. } => {
                if let Some(&owner) = tool_call_owner.get(tool_call_id) {
                    out.push(owner);
                }
            }
            ContentPart::Text { text } => {
                let file_refs = extract_text_file_tokens(text);
                for file in file_refs {
                    for (u, files) in files_per_turn.iter().enumerate().take(current_idx) {
                        if files.iter().any(|f| f == &file) {
                            out.push(u);
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
    }
    out
}

/// Re-use the relevance extractor on a synthetic single-text message
/// to get the file tokens. Duplicated logic is cheaper than exposing
/// the internal token-splitter.
fn extract_text_file_tokens(text: &str) -> Vec<String> {
    use crate::provider::Role;
    let synthetic = Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: text.to_string(),
        }],
    };
    extract(&synthetic).files
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::{ContentPart, Message, Role};

    fn text(role: Role, s: &str) -> Message {
        Message {
            role,
            content: vec![ContentPart::Text {
                text: s.to_string(),
            }],
        }
    }

    fn tool_call(id: &str, name: &str) -> Message {
        Message {
            role: Role::Assistant,
            content: vec![ContentPart::ToolCall {
                id: id.to_string(),
                name: name.to_string(),
                arguments: "{}".to_string(),
                thought_signature: None,
            }],
        }
    }

    fn tool_result(id: &str, body: &str) -> Message {
        Message {
            role: Role::Tool,
            content: vec![ContentPart::ToolResult {
                tool_call_id: id.to_string(),
                content: body.to_string(),
            }],
        }
    }

    #[test]
    fn oracle_binds_tool_results_to_their_call_owners() {
        let msgs = vec![
            text(Role::User, "do the thing"),
            tool_call("call-1", "Shell"),
            tool_result("call-1", "ok"),
        ];
        let report = replay_oracle(&msgs, 5);
        // Turn 2 (ToolResult) should reference turn 1 (its ToolCall).
        assert!(report.demand[2].contains(&1));
        assert!(report.reference_count() >= 1);
    }

    #[test]
    fn oracle_tracks_file_references_across_turns() {
        let msgs = vec![
            text(Role::User, "edit src/lib.rs"),
            text(Role::Assistant, "ok"),
            text(Role::User, "now open src/lib.rs again"),
        ];
        let report = replay_oracle(&msgs, 5);
        // Turn 2 re-mentions the file first seen at turn 0.
        assert!(report.demand[2].contains(&0));
    }

    #[test]
    fn oracle_respects_horizon_bound() {
        let msgs = vec![
            text(Role::User, "edit src/lib.rs"),
            text(Role::Assistant, "noop"),
            text(Role::Assistant, "noop"),
            text(Role::User, "reopen src/lib.rs"),
        ];
        let short = replay_oracle(&msgs, 1);
        let long = replay_oracle(&msgs, 10);
        // With a tiny horizon, turn 0's future window barely extends.
        assert!(long.reference_count() >= short.reference_count());
    }

    #[test]
    fn report_over_empty_trace_is_empty() {
        let report = replay_oracle(&[], 4);
        assert!(report.demand.is_empty());
        assert_eq!(report.horizon, 4);
        assert_eq!(report.reference_count(), 0);
    }
}
