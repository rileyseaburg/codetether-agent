//! Repository query filtering and aggregate statistics.
use super::*;
#[tokio::test]
async fn query_filters_and_statistics() {
    let (_dir, repo, first, run) = super::query_fixture::repository().await;
    assert_eq!(
        repo.query_okrs_by_owner("alice").await.unwrap()[0].id,
        first.id
    );
    assert_eq!(
        repo.query_okrs_by_tenant("tenant").await.unwrap()[0].id,
        first.id
    );
    assert_eq!(
        repo.query_okrs_by_status(OkrStatus::Active)
            .await
            .unwrap()
            .len(),
        1
    );
    assert!(repo.query_okrs_by_owner("bob").await.unwrap().is_empty());
    assert_eq!(
        repo.query_runs_by_okr(first.id).await.unwrap()[0].id,
        run.id
    );
    assert_eq!(
        repo.query_runs_by_status(OkrRunStatus::Draft)
            .await
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        repo.query_runs_by_correlation("correlation").await.unwrap()[0].id,
        run.id
    );
    assert_eq!(
        repo.query_runs_by_checkpoint("checkpoint").await.unwrap()[0].id,
        run.id
    );
    assert_eq!(
        repo.query_runs_by_session("session").await.unwrap()[0].id,
        run.id
    );
}
