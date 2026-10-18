use crate::*;

#[test]
fn stage_then_commit_appends_two_entries() {
    let mut journal = WritebackJournal::new("s1");
    let txn = journal.stage(Op::Compaction {
        before: 10,
        after: 3,
    });
    assert_eq!(journal.pending_count(), 1);
    journal.commit(txn).unwrap();
    assert_eq!(journal.pending_count(), 0);

    let entries = journal.entries();
    assert_eq!(entries.len(), 2);
    assert!(matches!(entries[0], JournalEntry::Staged { .. }));
    assert!(matches!(entries[1], JournalEntry::Committed { .. }));
}
