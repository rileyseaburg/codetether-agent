//! user text never pruned.
use super::*;

#[test]
pub(super) fn user_text_never_pruned() {
    let noisy = "a   b\n\n\n\nc";
    let mut msgs = vec![user(noisy)];
    for i in 0..KEEP_LAST_MESSAGES + 1 {
        msgs.push(asst(&format!("r{i}")));
    }
    let stats = prune_low_entropy(&mut msgs);
    assert_eq!(stats.total_bytes_saved, 0);
    let ContentPart::Text { text } = &msgs[0].content[0] else {
        panic!();
    };
    assert_eq!(text, noisy);
}
