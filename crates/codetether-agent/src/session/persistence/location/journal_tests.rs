//! Journals follow the durable snapshot rather than the caller's workspace.

use crate::session::journal::{JournalEntry, Op, WritebackJournal, append_entries};

#[tokio::test]
async fn journal_follows_relocated_snapshot() {
    let dir = tempfile::tempdir().unwrap();
    let id = uuid::Uuid::new_v4().to_string();
    let snapshot = dir.path().join(format!("{id}.json"));
    std::fs::write(&snapshot, b"{}").unwrap();
    super::record(&id, &snapshot);
    let mut journal = WritebackJournal::new(&id);
    let tx = journal.stage(Op::Save);
    journal.commit(tx).unwrap();
    append_entries(&id, journal.entries()).await.unwrap();
    let saved = std::fs::read_to_string(snapshot.with_extension("journal.jsonl")).unwrap();
    let entries: Vec<JournalEntry> = saved
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    assert_eq!(entries, journal.entries());
    super::forget(&id);
}

#[tokio::test]
async fn journal_rejects_path_traversal() {
    assert!(append_entries("../escape", &[]).await.is_err());
}
