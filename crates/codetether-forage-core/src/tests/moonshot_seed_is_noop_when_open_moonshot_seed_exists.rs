//! Shared forage regression coverage.
use super::*;

#[tokio::test]
async fn moonshot_seed_is_noop_when_open_moonshot_seed_exists() {
    let dir = tempdir().expect("create tempdir");
    let repo = codetether_okr::OkrRepository::new(dir.path().to_path_buf());

    let rubric = MoonshotRubric {
        goals: vec!["Tech stack is the moat".to_string()],
        required: true,
        min_alignment: 0.2,
    };
    let first = seed_moonshot_okr_if_no_opportunities(&repo, &rubric)
        .await
        .expect("first seed should succeed");
    assert!(first.is_some());

    let second = seed_moonshot_okr_if_no_opportunities(&repo, &rubric)
        .await
        .expect("second seed should succeed");
    assert!(
        second.is_none(),
        "should not duplicate active moonshot seed"
    );
}
