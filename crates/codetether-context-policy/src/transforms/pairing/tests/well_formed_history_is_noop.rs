//! Existing pairing regression.
use super::*;

#[test]
fn well_formed_history_is_noop() {
    let mut msgs = vec![user("q"), call("a"), result("a"), call("b"), result("b")];
    let before = msgs.len();
    let stats = repair_orphans(&mut msgs);
    assert_eq!(stats.snippet_hits, 0);
    assert_eq!(msgs.len(), before);
}
