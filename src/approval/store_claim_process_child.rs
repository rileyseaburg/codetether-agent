//! Child-process side of the approval claim race.

use super::{ApprovalStore, RESOURCE};
use std::io::Write;

#[test]
fn claim_child() {
    let Ok(id) = std::env::var("CODETETHER_CLAIM_CHILD") else {
        return;
    };
    if ApprovalStore::open_default()
        .expect("child store")
        .claim(&id, "bash", "execute", RESOURCE, "child")
        .is_ok()
    {
        let marker = std::env::var("CODETETHER_CLAIM_MARKER").expect("marker");
        writeln!(
            std::fs::OpenOptions::new()
                .append(true)
                .open(marker)
                .expect("open"),
            "claimed"
        )
        .expect("write");
    }
}
