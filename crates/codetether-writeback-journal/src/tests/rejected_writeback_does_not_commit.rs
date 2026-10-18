use crate::*;

#[test]
fn rejected_writeback_does_not_commit() {
    let mut journal = WritebackJournal::new("s1");
    let txn = journal.stage(Op::Reset);
    journal.reject(txn, RejectReason::DestructiveOp);

    assert_eq!(journal.pending_count(), 0);
    let entries = journal.entries();
    assert_eq!(entries.len(), 2);
    assert!(matches!(entries[1], JournalEntry::Rejected { .. }));

    // Commit on a rejected txn is an error.
    let err = journal.commit(txn).unwrap_err();
    assert!(matches!(err, RejectReason::SchemaMismatch { .. }));
}
