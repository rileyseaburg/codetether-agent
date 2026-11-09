//! Guards that the prompt teaches the exact format the parser accepts.
//!
//! Users reported non-stop tool failure on `gemini-web` models. Live runs showed
//! the model narrating tool use in prose ("I am calling the list tool...") while
//! emitting zero parseable blocks, because the protocol rules listed
//! prohibitions about `<tool_call>` without ever demonstrating the JSON shape.
//!
//! These tests fail if the demonstration is removed again.

use super::RULES;
use crate::GeminiWebProtocol;

#[test]
fn rules_demonstrate_the_required_json_shape() {
    assert!(RULES.contains("<tool_call>"), "must name the tag");
    assert!(RULES.contains("\"name\""), "must show the name field");
    assert!(
        RULES.contains("\"arguments\""),
        "must show the arguments field"
    );
    assert!(RULES.contains("invoke it even when you can compute the answer yourself"));
    assert!(RULES.contains("every requested tool step has returned a real result"));
}

#[test]
fn rules_teach_the_live_proven_sequential_executor_contract() {
    assert!(RULES.contains("real external tool executor"));
    assert!(RULES.contains("exactly one raw <tool_call> block, then STOP"));
    assert!(RULES.contains("Wait for the executor's real <tool_result>"));
    assert!(!RULES.contains("Emit zero to eight"));
}

#[test]
fn rules_forbid_narrating_instead_of_calling() {
    let lowered = RULES.to_lowercase();
    assert!(
        lowered.contains("prose"),
        "must tell the model that prose does not invoke a tool"
    );
}

#[test]
fn demonstrated_example_parses_as_a_real_tool_call() {
    // Extract the demonstrated block straight from the rules so the example can
    // never drift away from what `tool_calls::extract` accepts.
    let start = RULES.find("<tool_call>").expect("example block present");
    let end = RULES[start..]
        .find("</tool_call>")
        .expect("example block closed")
        + start
        + "</tool_call>".len();
    let example = &RULES[start..end];
    let body = example
        .strip_prefix("<tool_call>")
        .unwrap()
        .strip_suffix("</tool_call>")
        .unwrap();
    let parsed: serde_json::Value = serde_json::from_str(body).unwrap();
    assert!(parsed["arguments"].is_object());

    // The literal template uses placeholders, so substitute a concrete call
    // shaped exactly like the template before parsing.
    let concrete = example
        .replace("TOOL_NAME", "list")
        .replace("\"PARAMETER\":\"VALUE\"", "\"path\":\".\"");
    let (_, calls) = GeminiWebProtocol::extract_tool_calls(&concrete);
    assert_eq!(calls.len(), 1, "example must parse: {concrete}");
    assert_eq!(calls[0].0, "list");
}
