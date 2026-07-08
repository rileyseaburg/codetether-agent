//! Tests for palace sync and belief conversion.

use super::ledger::ScopeLedger;
use super::palace_belief;
use super::scope_item::ScopeItem;

fn item(deliverable: &str, status: &str) -> ScopeItem {
    ScopeItem {
        deliverable: deliverable.to_string(),
        status: status.to_string(),
        evidence_level: Some("static/local".to_string()),
        evidence: vec!["cargo test passed".to_string()],
    }
}

#[test]
fn belief_key_normalizes_deliverable() {
    assert_eq!(
        palace_belief::belief_key("Fix the Login-Bug!"),
        "fix-the-login-bug"
    );
    assert_eq!(palace_belief::belief_key(""), "");
}

#[test]
fn from_item_produces_active_high_confidence_belief() {
    let belief = palace_belief::from_item(&item("wire palace sync", "proven"), "sess-1");
    assert!(belief.confidence >= 0.7, "must clear injection threshold");
    assert_eq!(belief.claim, "wire palace sync");
    assert_eq!(belief.asserted_by, "session:sess-1");
    assert_eq!(belief.evidence_refs, vec!["cargo test passed"]);
}

#[test]
fn ledger_with_no_proven_items_is_skippable() {
    let ledger = ScopeLedger {
        session_id: "s".into(),
        items: vec![item("pending thing", "pending")],
        evidence: Vec::new(),
    };
    let proven = ledger.items.iter().filter(|i| i.status == "proven").count();
    assert_eq!(proven, 0);
}
