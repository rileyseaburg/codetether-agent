//! ellipsis only lines removed.
use super::*;

#[test]
pub(super) fn ellipsis_only_lines_removed() {
    let content = "real content\n...\n...\nmore content\n";
    let mut msgs = vec![asst(content)];
    for i in 0..KEEP_LAST_MESSAGES + 1 {
        msgs.push(user(&format!("q{i}")));
    }
    prune_low_entropy(&mut msgs);
    let ContentPart::Text { text } = &msgs[0].content[0] else {
        panic!();
    };
    assert!(!text.contains("..."));
    assert!(text.contains("real content"));
    assert!(text.contains("more content"));
}
