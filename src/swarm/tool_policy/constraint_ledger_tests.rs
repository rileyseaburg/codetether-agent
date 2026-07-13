use super::render;

#[test]
fn empty_sources_forbid_invented_bans() {
    let ledger = render("Refactor the graph", "", "", false);
    assert!(ledger.contains("No execution prohibitions were sourced"));
    assert!(ledger.contains("Do not invent test, build, compiler, linter, or watcher bans"));
}

#[test]
fn cites_current_task_instruction() {
    let ledger = render("Do not run tests.", "", "", false);
    assert!(ledger.contains("[delegated task instruction] Do not run tests."));
}

#[test]
fn cites_prior_dependency_context() {
    let ledger = render(
        "Refactor",
        "Prior decision: never run the watcher.",
        "",
        false,
    );
    assert!(ledger.contains("[prior dependency context]"));
}

#[test]
fn cites_repository_and_runtime_policy() {
    let ledger = render("Inspect", "", "Never run network commands.", true);
    assert!(ledger.contains("[repository policy (AGENTS.md)]"));
    assert!(ledger.contains("[runtime read-only mode]"));
}

#[test]
fn ordinary_test_mentions_are_not_prohibitions() {
    let ledger = render("Run focused tests after editing.", "", "", false);
    assert!(ledger.contains("No execution prohibitions were sourced"));
}
