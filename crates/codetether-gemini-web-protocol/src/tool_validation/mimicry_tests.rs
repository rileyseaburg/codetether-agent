use super::strip_trailing;

#[test]
fn strips_mimicked_result_after_a_real_call() {
    let text = concat!(
        r#"<tool_call>{"name":"read","arguments":{"path":"a.ts"}}</tool_call>"#,
        r#"<tool_result>{"tool_call_id":"t1","content":"fabricated"}</tool_result>"#,
    );

    let (kept, stripped) = strip_trailing(text);

    assert!(stripped);
    assert!(kept.contains("<tool_call>"));
    assert!(!kept.contains("tool_result"));
}

#[test]
fn leaves_clean_responses_untouched() {
    let (kept, stripped) = strip_trailing("Here is the answer.");

    assert!(!stripped);
    assert_eq!(kept, "Here is the answer.");
}

#[test]
fn drops_narration_that_follows_the_forged_block() {
    let text = "<tool_call>{}</tool_call><tool_result>{}</tool_result>then more prose";

    let (kept, _) = strip_trailing(text);

    assert!(!kept.contains("more prose"));
}

#[test]
fn handles_result_markup_with_attributes() {
    let (kept, stripped) = strip_trailing("<tool_call>{}</tool_call><tool_result id=\"x\">{}");

    assert!(stripped);
    assert_eq!(kept, "<tool_call>{}</tool_call>");
}

#[test]
fn forged_result_alone_yields_empty_text() {
    // Caller treats an empty remainder as a genuine forgery.
    let (kept, stripped) = strip_trailing("<tool_result>{\"content\":\"made up\"}</tool_result>");

    assert!(stripped);
    assert!(kept.is_empty());
}
