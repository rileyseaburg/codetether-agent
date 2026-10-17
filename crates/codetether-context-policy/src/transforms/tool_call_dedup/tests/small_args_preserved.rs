//! small args preserved.
use super::*;

#[test]
pub(super) fn small_args_preserved() {
    let mut msgs = vec![call("1", "noop", "{}"), call("2", "noop", "{}")];
    for _ in 0..KEEP_LAST_MESSAGES + 1 {
        msgs.push(Message {
            role: Role::User,
            content: vec![ContentPart::Text { text: "q".into() }],
        });
    }
    let stats = collapse_duplicate_calls(&mut msgs);
    assert_eq!(stats.dedup_hits, 0);
}
