use crate::session::helper::router::known_good_router_candidates;

#[test]
fn bedrock_fable_failure_prefers_bedrock_siblings_over_zai() {
    let candidates = known_good_router_candidates("bedrock", "us.anthropic.claude-fable-5");
    let first_bedrock = candidates
        .iter()
        .position(|c| c.starts_with("bedrock/"))
        .expect("bedrock sibling must be offered");
    let first_zai = candidates
        .iter()
        .position(|c| c.starts_with("zai/"))
        .expect("cross-provider tail still present");
    assert!(
        first_bedrock < first_zai,
        "bedrock sibling must rank before zai: {candidates:?}"
    );
}

#[test]
fn bedrock_ladder_excludes_the_failed_model() {
    let candidates = known_good_router_candidates("bedrock", "us.anthropic.claude-sonnet-4-6-v1:0");
    assert!(
        !candidates
            .iter()
            .any(|c| c.eq_ignore_ascii_case("bedrock/us.anthropic.claude-sonnet-4-6-v1:0")),
        "failed model must not be re-offered: {candidates:?}"
    );
    assert!(candidates.iter().any(|c| c.starts_with("bedrock/")));
}

#[test]
fn unknown_provider_still_gets_cross_provider_tail() {
    let candidates = known_good_router_candidates("bedrock-unknown", "some-model");
    assert!(candidates.iter().any(|c| c.starts_with("zai/")));
}
