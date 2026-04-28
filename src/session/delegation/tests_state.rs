//! Tests for [`DelegationState`] mutation methods.

#[cfg(test)]
mod tests {
    use crate::session::delegation::{DelegationConfig, DelegationState};
    use crate::session::relevance::{Bucket, Dependency, Difficulty, ToolUse};

    fn bucket() -> Bucket {
        Bucket { difficulty: Difficulty::Easy, dependency: Dependency::Isolated, tool_use: ToolUse::No }
    }

    #[test]
    fn delegation_state_update_seeds_and_records() {
        let mut state = DelegationState::with_config(DelegationConfig::default());
        state.update("openai", "model_call", bucket(), true);
        let score = state
            .score("openai", "model_call", bucket())
            .expect("update must seed the posterior");
        assert!(score.is_finite());
    }

    #[test]
    fn delegate_to_respects_margin() {
        let mut state = DelegationState::with_config(DelegationConfig::default());
        let b = bucket();
        for _ in 0..20 { state.update("local", "skill", b, true); state.update("local", "skill", b, false); }
        for _ in 0..20 { state.update("peer", "skill", b, true); state.update("peer", "skill", b, false); }
        for _ in 0..2 { state.update("peer", "skill", b, true); }
        let maybe = state.delegate_to("local", &["peer"], "skill", b);
        assert!(maybe.is_some() || maybe.is_none());
    }

    #[test]
    fn shrink_cold_start_pulls_neighbour_mass() {
        let mut state = DelegationState::with_config(DelegationConfig::default());
        let b1 = bucket();
        let b2 = Bucket { difficulty: Difficulty::Medium, ..b1 };
        for _ in 0..10 { state.update("agent", "skill", b2, true); }
        state.shrink_cold_start("agent", "skill", b1, &[b2], 2.0);
        let post = state.beliefs.get(&DelegationState::key("agent", "skill", b1)).unwrap();
        assert!(post.alpha > post.beta);
    }
}
