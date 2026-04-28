//! LCB-ordered model selection for autochat personas.
//!
//! Reorders a list of `"provider/model"` refs so that the
//! LCB-highest-scoring provider comes first.  When delegation is
//! disabled the input order is preserved unchanged.

use crate::session::delegation::DelegationState;
use crate::session::relevance::{Bucket, Dependency, Difficulty, ToolUse};

/// Skill constant used for autochat model-call delegation.
pub const AUTOCHAT_MODEL_CALL: &str = "autochat_model_call";

/// Sort `model_refs` by descending LCB score.
///
/// Each entry is `"provider/model"` — we extract the provider name,
/// look up its score, and sort highest-first.  Entries with no
/// posterior keep their relative input order (stable sort).
pub fn sort_refs_by_lcb(state: &DelegationState, model_refs: &mut [String], bucket: Bucket) {
    if !state.enabled() || model_refs.len() <= 1 {
        return;
    }
    model_refs.sort_by(|a, b| {
        let sa = score_ref(state, a, bucket);
        let sb = score_ref(state, b, bucket);
        sb.partial_cmp(&sa).unwrap_or(std::cmp::Ordering::Equal)
    });
}

fn score_ref(state: &DelegationState, model_ref: &str, bucket: Bucket) -> f64 {
    let (prov, _) = crate::provider::parse_model_string(model_ref);
    prov.and_then(|p| state.score(p, AUTOCHAT_MODEL_CALL, bucket))
        .unwrap_or(0.0)
}

/// Build a Bucket for a conversation turn.
pub fn bucket_for_turn(difficulty_hint: &str, uses_tools: bool) -> Bucket {
    let difficulty = match difficulty_hint {
        "hard" => Difficulty::Hard,
        "medium" => Difficulty::Medium,
        _ => Difficulty::Easy,
    };
    Bucket {
        difficulty,
        dependency: Dependency::Isolated,
        tool_use: if uses_tools {
            ToolUse::Yes
        } else {
            ToolUse::No
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::delegation::{DelegationConfig, DelegationState};

    #[test]
    fn sort_by_lcb_is_noop_when_disabled() {
        let state = DelegationState::with_config(DelegationConfig::default());
        let mut refs = vec!["b/model".into(), "a/model".into()];
        let b = bucket_for_turn("easy", false);
        sort_refs_by_lcb(&state, &mut refs, b);
        assert_eq!(refs[0], "b/model");
    }

    #[test]
    fn bucket_for_turn_maps_correctly() {
        let b = bucket_for_turn("hard", true);
        assert!(matches!(b.difficulty, Difficulty::Hard));
        assert!(matches!(b.tool_use, ToolUse::Yes));
    }
}
