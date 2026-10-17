//! Regression tests for Gemini card-placeholder rejection.

use super::latest;
use serde_json::json;

fn frame(text: &str) -> String {
    let inner = json!([null, null, null, null, [[null, [text]]]]);
    json!([["wrb.fr", null, inner.to_string()]]).to_string()
}

#[test]
fn trailing_card_placeholder_does_not_replace_the_answer() {
    let answer = "Here are the files you asked for.";
    let raw = [
        frame(answer),
        frame("http://googleusercontent.com/card_content/0"),
    ]
    .join("\n");
    assert_eq!(
        latest(&raw),
        answer,
        "a card placeholder must not win over real content"
    );
}

#[test]
fn card_placeholder_alone_yields_empty_rather_than_a_url() {
    let raw = frame("http://googleusercontent.com/card_content/0");
    assert!(
        latest(&raw).is_empty(),
        "placeholder-only response should not surface a URL as the answer"
    );
}

#[test]
fn text_mentioning_a_card_url_is_still_returned() {
    let text = "See http://googleusercontent.com/card_content/0 for the chart.";
    let raw = frame(text);
    assert_eq!(latest(&raw), text);
}

#[test]
fn tool_call_markup_still_wins_over_a_later_placeholder() {
    let call = "<tool_call>{\"name\":\"list\",\"arguments\":{\"path\":\".\"}}</tool_call>";
    let raw = [
        frame(call),
        frame("http://googleusercontent.com/card_content/2"),
    ]
    .join("\n");
    assert_eq!(latest(&raw), call);
}
