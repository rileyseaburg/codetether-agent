use super::{assertions::state, fixture::Fixture, git::run};
use crate::worktree::{WorktreeManager, maintenance::WorktreeCleanupState};

#[tokio::test]
async fn cleanup_removes_only_clean_merged_worktrees() {
    let fixture = Fixture::new();
    let merged = fixture.add("merged");
    let dirty = fixture.add("dirty");
    std::fs::write(dirty.join("draft"), "keep").unwrap();
    let unmerged = fixture.add("unmerged");
    std::fs::write(unmerged.join("change"), "new").unwrap();
    run(&unmerged, &["add", "change"]);
    run(&unmerged, &["commit", "-q", "-m", "change"]);
    let locked = fixture.add("locked");
    run(
        &fixture.repo,
        &["worktree", "lock", locked.to_str().unwrap()],
    );
    let manager = WorktreeManager::for_repo(&fixture.repo);

    let preview = manager
        .plan_merged_cleanup("main", std::slice::from_ref(&fixture.root))
        .await
        .unwrap();

    assert_eq!(state(&preview, "merged"), WorktreeCleanupState::Ready);
    assert_eq!(state(&preview, "dirty"), WorktreeCleanupState::Dirty);
    assert_eq!(state(&preview, "unmerged"), WorktreeCleanupState::Unmerged);
    assert_eq!(state(&preview, "locked"), WorktreeCleanupState::Locked);

    let report = manager
        .apply_merged_cleanup("main", &[fixture.root])
        .await
        .unwrap();
    assert_eq!(state(&report, "merged"), WorktreeCleanupState::Removed);
    assert!(!merged.exists());
    assert!(dirty.exists() && unmerged.exists() && locked.exists());
}
