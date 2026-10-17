//! Shared forage regression coverage.
use super::*;

#[tokio::test]
async fn moonshot_seed_creates_okr_when_no_opportunities_exist() {
    let dir = tempdir().expect("create tempdir");
    let repo = codetether_okr::OkrRepository::new(dir.path().to_path_buf());

    // Completed objective should not produce forage opportunities.
    let mut completed = Okr::new("Completed Objective", "Already done");
    completed.status = OkrStatus::Completed;
    let mut kr = KeyResult::new(completed.id, "KR done", 100.0, "%");
    kr.update_progress(100.0);
    completed.add_key_result(kr);
    let _ = repo.create_okr(completed).await.expect("create completed");

    let rubric = MoonshotRubric {
        goals: vec![
            "Automate customer acquisition end-to-end".to_string(),
            "Funnel conversion replaces manual sales".to_string(),
        ],
        required: true,
        min_alignment: 0.2,
    };

    let seeded_id = seed_moonshot_okr_if_no_opportunities(&repo, &rubric)
        .await
        .expect("seed should succeed");
    assert!(seeded_id.is_some(), "expected moonshot-derived seed OKR");
    let seeded_id = seeded_id.expect("seeded id");

    let okrs = repo.list_okrs().await.expect("list okrs");
    let seeded = okrs
        .iter()
        .find(|o| o.id == seeded_id)
        .expect("seeded okr exists");
    assert_eq!(seeded.status, OkrStatus::Active);
    assert_eq!(
        seeded.title,
        "Mission: Moonshot-Derived Autonomous Execution"
    );
    assert!(!seeded.key_results.is_empty());
}
