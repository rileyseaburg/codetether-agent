use super::is_preamble;
use crate::provider::ContentPart;

fn call(name: &str) -> ContentPart {
    ContentPart::ToolCall {
        id: "call_1".into(),
        name: name.into(),
        arguments: "{}".into(),
        thought_signature: None,
    }
}

fn text(body: &str) -> ContentPart {
    ContentPart::Text { text: body.into() }
}

#[test]
fn text_with_a_tool_call_is_preamble() {
    // GLM's chat template emits content and tool_calls in one turn; the live
    // failure was "Now let me read the actual files." becoming the answer.
    assert!(is_preamble(&[
        text("Now let me read the actual files."),
        call("read"),
    ]));
}

#[test]
fn tool_call_before_text_is_still_preamble() {
    assert!(is_preamble(&[call("grep"), text("Checking the routes.")]));
}

#[test]
fn text_alone_is_the_answer() {
    assert!(!is_preamble(&[text(
        "CatalogTabs is rendered on only two pages."
    )]));
}

#[test]
fn thinking_plus_text_without_a_call_is_the_answer() {
    let thinking = ContentPart::Thinking {
        text: "weighing options".into(),
        signature: None,
    };
    assert!(!is_preamble(&[thinking, text("Here is the finding.")]));
}

#[test]
fn empty_turn_is_not_preamble() {
    assert!(!is_preamble(&[]));
}

#[test]
fn parallel_tool_calls_are_preamble() {
    assert!(is_preamble(&[
        text("Starting parallel discovery."),
        call("glob"),
        call("grep"),
    ]));
}
