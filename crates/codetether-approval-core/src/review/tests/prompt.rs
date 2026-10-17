//! Prompt assembly for the reviewer.

use crate::review::{ReviewSubject, prompt};

#[test]
fn prompt_carries_goal_justification_and_diff() {
    let subject = ReviewSubject {
        tool: "apply_patch".into(),
        action: "write".into(),
        resource: "src/mux/model.rs".into(),
        justification: Some("split sessions per server".into()),
        preview: Some("-old\n+new".into()),
        goal: Some("## Goal Governance\nOBJECTIVE: isolate mux sessions".into()),
    };
    let text = prompt::user(&subject);
    for needle in [
        "OBJECTIVE: isolate mux sessions",
        "resource: src/mux/model.rs",
        "author's justification: split sessions per server",
        "```diff\n-old\n+new\n```",
    ] {
        assert!(text.contains(needle), "missing {needle:?}");
    }
}

#[test]
fn prompt_without_goal_or_preview_says_so() {
    let text = prompt::user(&ReviewSubject::default());
    assert!(text.contains("No goal is set"));
    assert!(text.contains("no preview was supplied"));
}
