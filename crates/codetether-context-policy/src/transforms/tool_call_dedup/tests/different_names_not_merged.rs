//! different names not merged.
use super::*;

#[test]
pub(super) fn different_names_not_merged() {
    let args = "a".repeat(200);
    let mut msgs = vec![
        call("1", "read_file", &args),
        call("2", "write_file", &args),
    ];
    for _ in 0..KEEP_LAST_MESSAGES + 1 {
        msgs.push(Message {
            role: Role::User,
            content: vec![ContentPart::Text { text: "q".into() }],
        });
    }
    let stats = collapse_duplicate_calls(&mut msgs);
    assert_eq!(stats.dedup_hits, 0);
}
