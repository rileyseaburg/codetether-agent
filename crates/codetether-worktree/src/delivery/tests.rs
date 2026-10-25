use crate::{DeliveryStatus, WorktreeManager, ledger_path};
use std::{fs, path::Path, process::Command};

fn git(repo: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn commit(repo: &Path, message: &str) {
    git(repo, &["add", "--", "file", "new.txt"]);
    git(repo, &["commit", "--no-gpg-sign", "-qm", message]);
}

async fn fixture() -> (tempfile::TempDir, std::path::PathBuf, WorktreeManager) {
    let dir = tempfile::tempdir().unwrap();
    let repo = dir.path().join("repo");
    fs::create_dir(&repo).unwrap();
    git(&repo, &["init", "-q", "-b", "main"]);
    git(&repo, &["config", "user.email", "test@example.com"]);
    git(&repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("file"), "base\n").unwrap();
    fs::write(repo.join("new.txt"), "seed\n").unwrap();
    commit(&repo, "base");
    let manager = WorktreeManager::for_repo(&repo).without_vscode_auto_open();
    let worker = manager.create("agent").await.unwrap();
    fs::write(worker.path.join("file"), "agent\n").unwrap();
    fs::write(worker.path.join("new.txt"), "worker\n").unwrap();
    commit(&worker.path, "worker change");
    (dir, repo, manager)
}

fn ledger_status(repo: &Path, id: i64) -> (String, i64) {
    let db = rusqlite::Connection::open(ledger_path(repo).unwrap()).unwrap();
    db.query_row(
        "SELECT status, length(patch) FROM deliveries WHERE id = ?1",
        [id],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )
    .unwrap()
}

#[tokio::test]
async fn applies_worker_changes_as_uncommitted_edits_only() {
    let (_dir, repo, manager) = fixture().await;
    let head = git(&repo, &["rev-parse", "HEAD"]);
    let delivery = manager.deliver("agent").await.unwrap();
    assert_eq!(
        delivery.status,
        DeliveryStatus::Applied,
        "{}",
        delivery.summary()
    );
    assert_eq!(
        git(&repo, &["rev-parse", "HEAD"]),
        head,
        "delivery must not commit"
    );
    assert!(
        git(&repo, &["diff", "--cached", "--name-only"]).is_empty(),
        "index untouched"
    );
    assert_eq!(fs::read_to_string(repo.join("file")).unwrap(), "agent\n");
    assert_eq!(delivery.files, ["file", "new.txt"]);
    let (status, patch_len) = ledger_status(&repo, delivery.id);
    assert_eq!(status, "applied");
    assert!(patch_len > 0, "ledger keeps the full patch");
}

#[tokio::test]
async fn conflicting_target_is_left_untouched_and_recorded() {
    let (_dir, repo, manager) = fixture().await;
    fs::write(repo.join("file"), "user edit\n").unwrap();
    let delivery = manager.deliver("agent").await.unwrap();
    assert_eq!(delivery.status, DeliveryStatus::Rejected);
    assert_eq!(
        fs::read_to_string(repo.join("file")).unwrap(),
        "user edit\n"
    );
    assert_eq!(
        fs::read_to_string(repo.join("new.txt")).unwrap(),
        "seed\n",
        "all-or-nothing"
    );
    assert_eq!(ledger_status(&repo, delivery.id).0, "rejected");
}

#[tokio::test]
async fn later_worker_commits_deliver_incrementally() {
    let (_dir, repo, manager) = fixture().await;
    manager.deliver("agent").await.unwrap();
    let worker = manager.get("agent").await.unwrap();
    fs::write(worker.path.join("file"), "agent v2\n").unwrap();
    commit(&worker.path, "worker follow-up");
    let second = manager.deliver("agent").await.unwrap();
    assert_eq!(
        second.status,
        DeliveryStatus::Applied,
        "{}",
        second.summary()
    );
    assert_eq!(second.files, ["file"]);
    assert_eq!(fs::read_to_string(repo.join("file")).unwrap(), "agent v2\n");
}

#[tokio::test]
async fn redelivery_is_idempotent() {
    let (_dir, repo, manager) = fixture().await;
    manager.deliver("agent").await.unwrap();
    let again = manager.deliver("agent").await.unwrap();
    assert_eq!(again.status, DeliveryStatus::AlreadyPresent);
    assert_eq!(fs::read_to_string(repo.join("file")).unwrap(), "agent\n");
}
