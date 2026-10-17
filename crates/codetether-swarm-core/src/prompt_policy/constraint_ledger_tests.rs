use super::{Mode, render};

const MUTATING: Mode = Mode::Mutating;

#[test]
fn empty_sources_forbid_invented_bans() {
    let ledger = render("Refactor the graph", "", "", MUTATING);
    assert!(ledger.contains("No execution prohibitions were sourced"));
    assert!(ledger.contains("Do not invent test, build, compiler, linter, or watcher bans"));
}

#[test]
fn cites_current_task_instruction() {
    let ledger = render("Do not run tests.", "", "", MUTATING);
    assert!(ledger.contains("[delegated task instruction] Do not run tests."));
}

#[test]
fn cites_prior_dependency_context() {
    let ledger = render(
        "Refactor",
        "Prior decision: never run the watcher.",
        "",
        MUTATING,
    );
    assert!(ledger.contains("[prior dependency context]"));
}

#[test]
fn cites_repository_and_runtime_policy() {
    let ledger = render("Inspect", "", "Never run network commands.", Mode::ReadOnly);
    assert!(ledger.contains("[repository policy (AGENTS.md)]"));
    assert!(ledger.contains("[runtime read-only mode]"));
}

#[test]
fn verification_mode_allows_commands_but_not_edits() {
    let ledger = render("Verify the goal", "", "", Mode::Verification);
    assert!(ledger.contains("[runtime verification mode]"));
    assert!(ledger.contains("Run read-only verification commands"));
    assert!(!ledger.contains("Do not run shell commands"));
}

#[test]
fn ordinary_test_mentions_are_not_prohibitions() {
    let ledger = render("Run focused tests after editing.", "", "", MUTATING);
    assert!(ledger.contains("No execution prohibitions were sourced"));
}
