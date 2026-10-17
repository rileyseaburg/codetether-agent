//! Incremental context selection around host-owned session and index access.
//!
//! [prepare] computes a selection plan using supplied estimators and a
//! protected-entry callback. After index acquisition, [finish] packs summaries,
//! repairs pairs and enforces the input budget.
use crate::pages::ResidencyLevel;
use crate::projection::helpers::DerivedContext;
use crate::projection::incremental_clamp::clamp_and_recompute;
use crate::projection::incremental_insert::interleave;
use crate::projection::incremental_repair::repair_with_origins;
use crate::projection::incremental_types::MessageOrigin;
use crate::selection::ranges::collect_dropped_ranges;
use crate::selection::{DEFAULT_RECENT_WINDOW, pack, score_messages, task_signature};
use crate::summary_index::SummaryIndex;
use codetether_provider_core::Message;
/// Prepared transcript selection awaiting a summary-index snapshot.
///
/// Construct this value through [prepare] and pass it to [finish].
///
/// # Examples
/// ```
/// use codetether_context_policy::derivation::incremental::{prepare, finish};
/// use codetether_context_policy::summary_index::SummaryIndex;
/// let plan = prepare(Vec::new(), 0, 100, |_| 0, |_| 0, |_, _, _, _| {});
/// let context = finish(plan, &SummaryIndex::new(), |_| 0, |_| 0);
/// assert!(context.messages.is_empty());
/// ```
pub struct PreparedIncremental {
    clone: Vec<Message>,
    origin_len: usize,
    budget_tokens: usize,
    keep: Vec<bool>,
    per_msg: Vec<usize>,
    order: Vec<usize>,
    budget_for_messages: usize,
}
struct ProjectedIncremental {
    messages: Vec<Message>,
    origins: Vec<MessageOrigin>,
    resolutions: Vec<ResidencyLevel>,
    dropped_ranges: Vec<(usize, usize)>,
    clone_len: usize,
    origin_len: usize,
    budget_tokens: usize,
}
mod prepare;
pub use prepare::prepare;
mod assemble;
mod finish;
pub use finish::finish;

mod residency;
use residency::residency_for_message;
