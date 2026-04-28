//! [`DerivePolicy::Incremental`] — Liu et al. select-then-pack derivation.
//!
//! Reference: arXiv:2512.22087 §3 (history vs. context separation),
//! §4 (incremental selection over a relevance sidecar).
//!
//! ## Algorithm
//!
//! 1. Cheap path: if the full clone already fits the token budget, return it.
//! 2. Build a **task signature** from the most recent user turn's
//!    [`RelevanceMeta`] (files, tools, error classes).
//! 3. Score every message:
//!    * `+∞` for entries inside the **recent window** (always kept).
//!    * `+file_overlap` against the task signature.
//!    * `+tool_overlap`.
//!    * `+error_signal` (boundary boost).
//!    * `+recency_decay`.
//! 4. Greedy pack in score order until the per-message token estimate
//!    drains the remaining budget.
//! 5. Re-sort selected entries by original index (causal order).
//! 6. Run [`pairing::repair_orphans`] as a final safety pass to ensure
//!    every `ToolCall` keeps its matching `ToolResult` (the score-based
//!    selection can otherwise split a pair).
//!
//! Step 18 will fill the gaps left by selection with cached summaries
//! from [`SummaryIndex`]; step 14's index returns `None` for every
//! lookup, so today the selected entries simply telescope.

use std::sync::Arc;

use anyhow::Result;

use crate::provider::{Message, Role, ToolDefinition};
use crate::session::ResidencyLevel;
use crate::session::Session;
use crate::session::helper::experimental;
use crate::session::helper::token::{estimate_request_tokens, estimate_tokens_for_messages};
use crate::session::relevance::{RelevanceMeta, extract};

use super::helpers::DerivedContext;

/// Default recent-window size — last N entries always retained.
const DEFAULT_RECENT_WINDOW: usize = 8;

/// Per-overlap-file weight in the relevance score.
const FILE_OVERLAP_WEIGHT: f64 = 4.0;

/// Per-overlap-tool weight in the relevance score.
const TOOL_OVERLAP_WEIGHT: f64 = 2.5;

/// Per-error-class weight (boundary boost).
const ERROR_BOUNDARY_WEIGHT: f64 = 3.0;

/// Recency decay weight: each step away from the tail subtracts this much.
const RECENCY_DECAY_WEIGHT: f64 = 0.05;

/// [`DerivePolicy::Incremental`](crate::session::derive_policy::DerivePolicy::Incremental)
/// implementation.
pub(super) async fn derive_incremental(
    session: &Session,
    _provider: Arc<dyn crate::provider::Provider>,
    _model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    budget_tokens: usize,
) -> Result<DerivedContext> {
    let origin_len = session.messages.len();
    let mut clone = session.messages.clone();

    let full_estimate = estimate_request_tokens(system_prompt, &clone, tools);
    if full_estimate <= budget_tokens {
        experimental::pairing::repair_orphans(&mut clone);
        return Ok(DerivedContext {
            resolutions: vec![ResidencyLevel::Full; clone.len()],
            dropped_ranges: Vec::new(),
            provenance: vec!["incremental_below_budget".to_string()],
            messages: clone,
            origin_len,
            compressed: false,
        });
    }

    let task = task_signature(&clone);
    let scores = score_messages(&clone, &task);

    let recent_window = std::cmp::min(DEFAULT_RECENT_WINDOW, clone.len());
    let recent_start = clone.len() - recent_window;

    let header_cost = budget_tokens
        .saturating_sub(estimate_request_tokens(system_prompt, &[], tools))
        .max(1);
    // Token budget for the message slice itself, after system + tools.
    let mut budget_for_messages = header_cost;

    let per_msg = clone
        .iter()
        .map(|m| message_tokens(m))
        .collect::<Vec<usize>>();

    // Always-include set: the recent-window entries.
    let mut keep = vec![false; clone.len()];
    for (i, slot) in keep.iter_mut().enumerate().take(clone.len()).skip(recent_start) {
        *slot = true;
        budget_for_messages = budget_for_messages.saturating_sub(per_msg[i]);
    }

    // Greedy pack the older entries by descending score.
    let mut order: Vec<usize> = (0..recent_start).collect();
    order.sort_by(|a, b| {
        scores[*b]
            .partial_cmp(&scores[*a])
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    for idx in order {
        let cost = per_msg[idx];
        if cost <= budget_for_messages {
            keep[idx] = true;
            budget_for_messages = budget_for_messages.saturating_sub(cost);
        }
    }

    let dropped_ranges = collect_dropped_ranges(&keep);
    let mut messages: Vec<Message> = clone
        .into_iter()
        .zip(keep.iter().copied())
        .filter_map(|(m, k)| if k { Some(m) } else { None })
        .collect();

    experimental::pairing::repair_orphans(&mut messages);

    // entries back in and busted the budget, trim from the oldest side
    // until the request fits. This covers both the case where pairing
    // repair re-introduced entries and where the recent window itself
    // exceeds the budget.
    let mut post_pair_estimate = estimate_request_tokens(system_prompt, &messages, tools);
    let mut provenance = vec!["incremental".to_string()];
    while post_pair_estimate > budget_tokens && messages.len() > 1 {
        messages.remove(0);
        provenance.push("incremental_overflow_clamp".to_string());
        post_pair_estimate = estimate_request_tokens(system_prompt, &messages, tools);
    }

    Ok(DerivedContext {
        resolutions: vec![ResidencyLevel::Full; messages.len()],
        dropped_ranges,
        provenance,
        messages,
        origin_len,
        compressed: true,
    })
}

/// Build a relevance signature from the most recent user turn.
fn task_signature(messages: &[Message]) -> RelevanceMeta {
    messages
        .iter()
        .rev()
        .find(|m| matches!(m.role, Role::User))
        .map(extract)
        .unwrap_or_default()
}

/// Score every message against `task`. Higher is more relevant.
fn score_messages(messages: &[Message], task: &RelevanceMeta) -> Vec<f64> {
    let n = messages.len();
    messages
        .iter()
        .enumerate()
        .map(|(i, msg)| {
            let meta = extract(msg);
            let mut score = 0.0;
            score += FILE_OVERLAP_WEIGHT * overlap_count(&meta.files, &task.files) as f64;
            score += TOOL_OVERLAP_WEIGHT * overlap_count(&meta.tools, &task.tools) as f64;
            score += ERROR_BOUNDARY_WEIGHT * meta.error_classes.len() as f64;
            // Recency: newer messages get a small bonus regardless of overlap.
            let distance_from_tail = (n - 1).saturating_sub(i) as f64;
            score -= RECENCY_DECAY_WEIGHT * distance_from_tail;
            score
        })
        .collect()
}

fn overlap_count(left: &[String], right: &[String]) -> usize {
    if left.is_empty() || right.is_empty() {
        return 0;
    }
    let right_set: std::collections::HashSet<_> = right.iter().collect();
    left.iter().filter(|item| right_set.contains(item)).count()
}

fn message_tokens(msg: &Message) -> usize {
    estimate_tokens_for_messages(std::slice::from_ref(msg))
}

fn collect_dropped_ranges(keep: &[bool]) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut i = 0;
    while i < keep.len() {
        if !keep[i] {
            let start = i;
            while i < keep.len() && !keep[i] {
                i += 1;
            }
            ranges.push((start, i));
        } else {
            i += 1;
        }
    }
    ranges
}

/// Default per-policy budget when the variant carries `budget_tokens: 0`.
pub(super) const DEFAULT_INCREMENTAL_BUDGET: usize = 16_000;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::{ContentPart, Role};

    fn user(text: &str) -> Message {
        Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: text.to_string(),
            }],
        }
    }

    fn assistant(text: &str) -> Message {
        Message {
            role: Role::Assistant,
            content: vec![ContentPart::Text {
                text: text.to_string(),
            }],
        }
    }

    #[test]
    fn task_signature_picks_up_files_from_last_user_turn() {
        let msgs = vec![
            assistant("noise"),
            user("Please edit src/lib.rs and crates/foo/main.rs"),
            assistant("ok"),
        ];
        let task = task_signature(&msgs);
        assert!(task.files.iter().any(|f| f == "src/lib.rs"));
        assert!(task.files.iter().any(|f| f == "crates/foo/main.rs"));
    }

    #[test]
    fn score_messages_rewards_file_overlap() {
        let msgs = vec![
            assistant("looking at src/lib.rs"), // overlap
            assistant("totally unrelated noise"),
            user("touch src/lib.rs"),
        ];
        let task = task_signature(&msgs);
        let scores = score_messages(&msgs, &task);
        // The earlier overlapping message scores higher than the noise.
        assert!(scores[0] > scores[1]);
    }

    #[test]
    fn collect_dropped_ranges_groups_consecutive_drops() {
        let keep = vec![true, false, false, true, false, true];
        assert_eq!(collect_dropped_ranges(&keep), vec![(1, 3), (4, 5)]);
    }

    #[test]
    fn collect_dropped_ranges_empty_when_all_kept() {
        let keep = vec![true, true, true];
        assert_eq!(collect_dropped_ranges(&keep), Vec::<(usize, usize)>::new());
    }

    #[test]
    fn overlap_count_short_circuits_on_empty() {
        let left: Vec<String> = vec!["a".into()];
        let right: Vec<String> = Vec::new();
        assert_eq!(overlap_count(&left, &right), 0);
    }
}
