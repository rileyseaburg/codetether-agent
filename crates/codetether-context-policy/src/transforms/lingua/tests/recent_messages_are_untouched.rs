//! recent messages are untouched.
use super::*;

#[test]
pub(super) fn recent_messages_are_untouched() {
    let noisy = "a  b\n\n\n\nc";
    let mut msgs = vec![asst(noisy)];
    // Not enough history — noisy stays.
    let stats = prune_low_entropy(&mut msgs);
    assert_eq!(stats.total_bytes_saved, 0);
}
