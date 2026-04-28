//! Tests for `rank_candidates` and config defaults.

#[cfg(test)]
mod tests_rank {
    use crate::session::delegation::{DelegationConfig, DelegationState};
    use crate::session::relevance::{Bucket, Dependency, Difficulty, ToolUse};

    fn bucket() -> Bucket {
        Bucket { difficulty: Difficulty::Easy, dependency: Dependency::Isolated, tool_use: ToolUse::No }
    }

    #[test]
    fn rank_candidates_picks_first_on_cold_start() {
        let state = DelegationState::with_config(DelegationConfig::default());
        assert_eq!(state.rank_candidates(&["a", "b", "c"], "swarm_dispatch", bucket()), Some("a"));
    }

    #[test]
    fn rank_candidates_prefers_best_scoring_once_warm() {
        let mut state = DelegationState::with_config(DelegationConfig::default());
        let b = bucket();
        for _ in 0..5 { state.update("b", "swarm_dispatch", b, true); }
        for _ in 0..5 { state.update("a", "swarm_dispatch", b, false); }
        assert_eq!(state.rank_candidates(&["a", "b"], "swarm_dispatch", b), Some("b"));
    }

    #[test]
    fn rank_candidates_is_none_for_empty_input() {
        let state = DelegationState::with_config(DelegationConfig::default());
        assert!(state.rank_candidates(&[], "swarm_dispatch", bucket()).is_none());
    }
}

#[cfg(test)]
mod tests_config {
    use crate::session::delegation::{DelegationConfig, DEFAULT_DELTA, DEFAULT_GAMMA, DEFAULT_KAPPA, DEFAULT_LAMBDA};

    #[test]
    fn config_defaults_match_documented_constants() {
        let cfg = DelegationConfig::default();
        assert!((cfg.gamma - DEFAULT_GAMMA).abs() < 1e-9);
        assert!((cfg.delta - DEFAULT_DELTA).abs() < 1e-9);
        assert!((cfg.kappa - DEFAULT_KAPPA).abs() < 1e-9);
        assert!((cfg.lambda - DEFAULT_LAMBDA).abs() < 1e-9);
        assert!(!cfg.enabled);
    }
}
