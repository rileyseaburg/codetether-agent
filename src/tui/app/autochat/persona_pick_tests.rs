//! Tests for [`super::persona_pick`].

use super::persona::default_chain;
use super::persona_pick::{rank_chain, relay_bucket};
use crate::session::delegation::{DelegationConfig, DelegationState};
use crate::session::delegation_skills::AUTOCHAT_PERSONA;

#[test]
fn disabled_returns_static_chain() {
    let state = DelegationState::default();
    let chain = rank_chain(&state, relay_bucket());
    assert_eq!(chain[0].name, default_chain()[0].name);
}

#[test]
fn enabled_promotes_high_scoring_persona() {
    let mut state = DelegationState::with_config(DelegationConfig {
        enabled: true,
        ..DelegationConfig::default()
    });
    for _ in 0..10 {
        state.update("reviewer", AUTOCHAT_PERSONA, relay_bucket(), true);
    }
    let chain = rank_chain(&state, relay_bucket());
    assert_eq!(chain[0].name, "reviewer");
}
