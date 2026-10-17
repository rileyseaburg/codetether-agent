//! short history is noop.
use super::*;

#[test]
pub(super) fn short_history_is_noop() {
    let mut msgs = vec![u("a"), a("b"), u("c"), a("d")];
    let before = msgs.clone();
    let stats = trim_middle(&mut msgs);
    assert_eq!(stats.total_bytes_saved, 0);
    assert_eq!(msgs.len(), before.len());
}
