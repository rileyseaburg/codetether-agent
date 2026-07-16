use super::{assertions::state, fixture::Fixture, git::run};
use crate::worktree::{WorktreeManager, maintenance::WorktreeCleanupState};

#[tokio::test]
async fn stale_registration_requires_a_merged_commit() {
    let fixture = Fixture::new();
    let merged = fixture.add("merged-missing");
    std::fs::remove_dir_all(&merged).unwrap();
    let unmerged = fixture.add("unmerged-missing");
    std::fs::write(unmerged.join("change"), "new").unwrap();
    run(&unmerged, &["add", "change"]);
    run(&unmerged, &["commit", "-q", "-m", "change"]);
    std::fs::remove_dir_all(&unmerged).unwrap();
    let manager = WorktreeManager::for_repo(&fixture.repo);

    let preview = manager
        .plan_merged_cleanup("main", std::slice::from_ref(&fixture.root))
        .await
        .unwrap();
    assert_eq!(
        state(&preview, "merged-missing"),
        WorktreeCleanupState::Prunable
    );
    assert_eq!(
        state(&preview, "unmerged-missing"),
        WorktreeCleanupState::Unmerged
    );

    let report = manager
        .apply_merged_cleanup("main", &[fixture.root])
        .await
        .unwrap();
    assert_eq!(
        state(&report, "merged-missing"),
        WorktreeCleanupState::Removed
    );
    assert_eq!(
        state(&report, "unmerged-missing"),
        WorktreeCleanupState::Unmerged
    );
}
