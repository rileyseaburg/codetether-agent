use super::render;

#[test]
fn refactor_requires_fresh_before_and_after_metrics() {
    let prompt = render("refactor", "Split GraphCanvas.tsx", false);
    assert!(prompt.contains("Immediately before planning or editing"));
    assert!(prompt.contains("current workspace"));
    assert!(prompt.contains("physical line count"));
    assert!(prompt.contains("excluding comments and blank lines"));
    assert!(prompt.contains("naming the command or tool used"));
    assert!(prompt.contains("rerun the same measurement"));
    assert!(prompt.contains("per-file before/after counts"));
}

#[test]
fn rejects_remembered_approximate_counts_as_evidence() {
    let prompt = render(
        "generalist",
        "Refactor the roughly 442-line component",
        false,
    );
    assert!(prompt.contains("remembered, approximate, or prior-session counts"));
    assert!(prompt.contains("never as current evidence"));
}

#[test]
fn non_refactor_and_read_only_tasks_are_unchanged() {
    assert!(render("tester", "Run focused tests", false).is_empty());
    assert!(render("refactor", "Inspect structure", true).is_empty());
}
