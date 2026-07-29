//! Guards that the prompt teaches the exact format the parser accepts.
//!
//! Users reported non-stop tool failure on `gemini-web` models. Live runs showed
//! the model narrating tool use in prose ("I am calling the list tool...") while
//! emitting zero parseable blocks, because the protocol rules listed
//! prohibitions about `<tool_call>` without ever demonstrating the JSON shape.
//!
//! These tests fail if the demonstration is removed again.

use super::RULES;
use crate::provider::gemini_web::GeminiWebProvider;

#[test]
fn rules_demonstrate_the_required_json_shape() {
    assert!(RULES.contains("<tool_call>"), "must name the tag");
    assert!(RULES.contains("\"name\""), "must show the name field");
    assert!(
        RULES.contains("\"arguments\""),
        "must show the arguments field"
    );
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

    // The literal template uses placeholders, so substitute a concrete call
    // shaped exactly like the template before parsing.
    let concrete = example
        .replace("<tool name>", "list")
        .replace("<JSON arguments>", "\"path\": \".\"");
    let (_, calls) = GeminiWebProvider::extract_tool_calls(&concrete);
    assert_eq!(calls.len(), 1, "example must parse: {concrete}");
    assert_eq!(calls[0].0, "list");
}
