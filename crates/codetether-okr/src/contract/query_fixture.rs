//! Populated repository fixture for query tests.
use super::*;
pub(super) async fn repository() -> (tempfile::TempDir, OkrRepository, Okr, OkrRun) {
    let dir = tempfile::tempdir().unwrap();
    let repo = OkrRepository::new(dir.path().into());
    let mut first = objective();
    first.owner = Some("alice".into());
    first.tenant_id = Some("tenant".into());
    first.status = OkrStatus::Active;
    let first = repo.create_okr(first).await.unwrap();
    repo.create_okr(objective()).await.unwrap();
    let mut run = OkrRun::new(first.id, "run");
    run.correlation_id = Some("correlation".into());
    run.relay_checkpoint_id = Some("checkpoint".into());
    run.session_id = Some("session".into());
    let run = repo.create_run(run).await.unwrap();

    (dir, repo, first, run)
}
