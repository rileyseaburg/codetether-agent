use crate::*;
use chrono::Utc;

#[test]
fn journal_entry_round_trips_through_serde() {
    let entry = JournalEntry::Rejected {
        txn: TxnId(7),
        at: Utc::now(),
        reason: RejectReason::InvariantViolation {
            detail: "constraint below structured".into(),
        },
    };
    let json = serde_json::to_string(&entry).unwrap();
    let back: JournalEntry = serde_json::from_str(&json).unwrap();
    assert_eq!(entry, back);
}
