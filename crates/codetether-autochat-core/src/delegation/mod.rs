//! autochat delegation contracts and policy operations.
//! LCB-ordered model selection for autochat personas.
//!
//! Reorders a list of `"provider/model"` refs so that the
//! LCB-highest-scoring provider comes first.  When delegation is
//! disabled the input order is preserved unchanged.

use codetether_context_policy::delegation::DelegationState;

use codetether_context_policy::relevance::{Bucket, Dependency, Difficulty, ToolUse};

/// Skill constant used for autochat model-call delegation.
pub const AUTOCHAT_MODEL_CALL: &str = "autochat_model_call";
mod fn_sort_refs_by_lcb;
pub use fn_sort_refs_by_lcb::sort_refs_by_lcb;
mod fn_score_ref;
use fn_score_ref::score_ref;
mod fn_bucket_for_turn;
pub use fn_bucket_for_turn::bucket_for_turn;
#[cfg(test)]
mod tests;
