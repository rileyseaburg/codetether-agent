use super::select;

fn owned(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}

#[test]
fn repeated_candidate_beats_single_stray_frame() {
    // The live 40 KB tool-result shape: answer repeats, junk appears once last.
    let picked = select(&owned(&[
        "TOOL_TURN_OK",
        "TOOL_TURN_OK",
        "TOOL_TURN_OK",
        "Call center agent: Yes",
    ]));
    assert_eq!(picked.as_deref(), Some("TOOL_TURN_OK"));
}

#[test]
fn leaked_reasoning_appearing_once_is_rejected() {
    let picked = select(&owned(&[
        "I will inspect the files.",
        "RUNBOOK_OK",
        "RUNBOOK_OK",
        "RUNBOOK_OK",
    ]));
    assert_eq!(picked.as_deref(), Some("RUNBOOK_OK"));
}

#[test]
fn three_way_repeat_wins_over_two_way() {
    let picked = select(&owned(&["a", "b", "a", "b", "a"]));
    assert_eq!(picked.as_deref(), Some("a"));
}

#[test]
fn short_stream_keeps_last_wins_behaviour() {
    // A two-frame reply legitimately shows each candidate once.
    assert_eq!(
        select(&owned(&["draft", "final"])).as_deref(),
        Some("final")
    );
    assert_eq!(select(&owned(&["a", "b", "c"])).as_deref(), Some("c"));
}

#[test]
fn long_stream_without_agreement_falls_back_to_order() {
    let picked = select(&owned(&["one", "two", "three", "four", "five"]));
    assert_eq!(picked.as_deref(), Some("five"));
}

#[test]
fn single_and_empty_inputs() {
    assert_eq!(select(&owned(&["only"])).as_deref(), Some("only"));
    assert!(select(&[]).is_none());
}
