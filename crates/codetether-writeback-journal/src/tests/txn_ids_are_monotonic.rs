use crate::*;

#[test]
fn txn_ids_are_monotonic() {
    let mut journal = WritebackJournal::new("s1");
    let a = journal.stage(Op::Save);
    let b = journal.stage(Op::Save);
    assert!(b.0 > a.0);
}
