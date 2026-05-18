use std::path::Path;

pub(crate) fn render(cwd: &Path) -> String {
    let mut facts = Vec::new();
    if cwd.join(".git").exists() {
        facts.push("git workspace detected");
    }
    if cwd.join(".codetether/memory.json").exists() {
        facts.push("project memory palace present");
    }
    if cwd.join(".codetether/session-ledgers").exists() {
        facts.push("session scope ledgers present");
    }
    if facts.is_empty() {
        "Runtime prefetch facts: none loaded.".to_string()
    } else {
        format!("Runtime prefetch facts: {}.", facts.join("; "))
    }
}
