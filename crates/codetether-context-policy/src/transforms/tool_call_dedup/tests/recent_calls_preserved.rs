//! recent calls preserved.
use super::*;

#[test]
pub(super) fn recent_calls_preserved() {
    let args = "z".repeat(200);
    let mut msgs = vec![call("1", "f", &args)];
    for _ in 0..KEEP_LAST_MESSAGES - 1 {
        msgs.push(Message {
            role: Role::User,
            content: vec![ContentPart::Text { text: "q".into() }],
        });
    }
    msgs.push(call("2", "f", &args));
    let stats = collapse_duplicate_calls(&mut msgs);
    assert_eq!(stats.dedup_hits, 0);
}
