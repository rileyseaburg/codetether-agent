//! code fences are preserved.
use super::*;

#[test]
pub(super) fn code_fences_are_preserved() {
    let content = "text  with   spaces\n```\nfn  foo() {}\n```\ntail";
    let mut msgs = vec![asst(content)];
    for i in 0..KEEP_LAST_MESSAGES + 1 {
        msgs.push(user(&format!("q{i}")));
    }
    prune_low_entropy(&mut msgs);
    let ContentPart::Text { text } = &msgs[0].content[0] else {
        panic!();
    };
    // Outside fence: collapsed.
    assert!(text.contains("text with spaces"));
    // Inside fence: preserved verbatim.
    assert!(text.contains("fn  foo() {}"));
}
