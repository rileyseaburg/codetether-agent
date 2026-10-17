//! Repository test_crud_runs regression.

use super::*;

#[tokio::test]
async fn test_crud_runs() {
    let repo = temp_repo();

    // First create an OKR
    let mut okr = Okr::new("Test OKR", "Desc");
    let okr_id = okr.id;
    let kr = KeyResult::new(okr.id, "KR1", 100.0, "%");
    okr.add_key_result(kr);
    repo.create_okr(okr).await.unwrap();

    // Create a run
    let mut run = OkrRun::new(okr_id, "Q1 Run");
    run.correlation_id = Some("corr-123".to_string());
    run.session_id = Some("session-456".to_string());

    let created = repo.create_run(run).await.unwrap();

    // Fetch by ID
    let fetched = repo.get_run(created.id).await.unwrap();
    assert!(fetched.is_some());
    assert_eq!(fetched.unwrap().name, "Q1 Run");

    // Query by OKR
    let runs_for_okr = repo.query_runs_by_okr(okr_id).await.unwrap();
    assert_eq!(runs_for_okr.len(), 1);

    // Query by correlation
    let runs_by_corr = repo.query_runs_by_correlation("corr-123").await.unwrap();
    assert_eq!(runs_by_corr.len(), 1);

    // Query by status
    let pending = repo
        .query_runs_by_status(OkrRunStatus::Draft)
        .await
        .unwrap();
    assert_eq!(pending.len(), 1);
}
