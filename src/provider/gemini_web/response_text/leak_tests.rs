use super::content_parts;
use crate::provider::ContentPart;

#[test]
fn captures_leaked_reasoning_and_keeps_the_answer() {
    // Exact shape captured from gemini-web-fast: there is no separator between
    // two reasoning sentences and the real answer.
    let text = concat!(
        "I will run a glob check to explore the project structure.",
        "I am inspecting the workspace to check the repository status.",
        "Here is a breakdown of the differences between A2A and MCP."
    );
    let parts = content_parts(text);
    assert!(matches!(
        &parts[0], ContentPart::Thinking { text, signature: None }
        if text == concat!(
            "I will run a glob check to explore the project structure.",
            "I am inspecting the workspace to check the repository status."
        )
    ));
    assert!(matches!(
        &parts[1], ContentPart::Text { text }
        if text == "Here is a breakdown of the differences between A2A and MCP."
    ));
}

#[test]
fn strips_followup_ui_without_discarding_answer() {
    let text = concat!(
        "Final answer.",
        "\n<FollowUp label=\"Would you like an example?\" ",
        "query=\"Show me an example.\"/>"
    );
    let parts = content_parts(text);
    assert!(matches!(
        &parts[..],
        [ContentPart::Text { text }] if text == "Final answer."
    ));
}
